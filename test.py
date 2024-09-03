import win32com.client
import inspect
import pythoncom

def get_type_name(type_id):
    type_names = {
        pythoncom.VT_I2: "short", pythoncom.VT_I4: "long", pythoncom.VT_R4: "float",
        pythoncom.VT_R8: "double", pythoncom.VT_CY: "currency", pythoncom.VT_DATE: "date",
        pythoncom.VT_BSTR: "string", pythoncom.VT_DISPATCH: "IDispatch",
        pythoncom.VT_ERROR: "error", pythoncom.VT_BOOL: "boolean",
        pythoncom.VT_VARIANT: "variant", pythoncom.VT_UNKNOWN: "IUnknown",
        pythoncom.VT_I1: "char", pythoncom.VT_UI1: "unsigned char",
        pythoncom.VT_UI2: "unsigned short", pythoncom.VT_UI4: "unsigned long",
        pythoncom.VT_I8: "int64", pythoncom.VT_UI8: "unsigned int64",
        pythoncom.VT_INT: "int", pythoncom.VT_UINT: "unsigned int",
        pythoncom.VT_VOID: "void", pythoncom.VT_HRESULT: "HRESULT",
        pythoncom.VT_PTR: "pointer", pythoncom.VT_SAFEARRAY: "SafeArray",
        pythoncom.VT_CARRAY: "C Array", pythoncom.VT_USERDEFINED: "user defined",
        pythoncom.VT_LPSTR: "LPSTR", pythoncom.VT_LPWSTR: "LPWSTR",
        pythoncom.VT_RECORD: "record", pythoncom.VT_FILETIME: "FILETIME",
        pythoncom.VT_BLOB: "BLOB", pythoncom.VT_STREAM: "IStream",
        pythoncom.VT_STORAGE: "IStorage", pythoncom.VT_STREAMED_OBJECT: "streamed object",
        pythoncom.VT_STORED_OBJECT: "stored object", pythoncom.VT_BLOB_OBJECT: "BLOB object",
        pythoncom.VT_CF: "CF", pythoncom.VT_CLSID: "CLSID"
    }
    return type_names.get(type_id, "unknown")

def analyze_com_object(com_name):
    obj = win32com.client.Dispatch(com_name)
    methods = []
    properties = []

    try:
        type_info = obj._oleobj_.GetTypeInfo()
        attr = type_info.GetTypeAttr()
        for i in range(attr[6]):
            func_desc = type_info.GetFuncDesc(i)
            func_name = type_info.GetNames(func_desc[0])[0]

            if func_desc[3] == pythoncom.INVOKE_FUNC:
                param_info = [get_type_name(desc[1]) for desc in func_desc[2]]
                return_type = get_type_name(func_desc[8])
                methods.append((func_name, param_info, return_type))
            elif func_desc[3] & pythoncom.INVOKE_PROPERTYGET:
                prop_type = get_type_name(func_desc[8])
                properties.append((func_name, prop_type))
    except:
        print("无法获取详细类型信息,将使用基本信息。")
        for item in dir(obj):
            if item.startswith('_'):
                continue
            try:
                attr = getattr(obj, item)
                if callable(attr):
                    methods.append((item, [], "unknown"))
                else:
                    properties.append((item, type(attr).__name__))
            except:
                pass

    print(f"COM对象: {com_name}")
    print(f"\n方法 (共{len(methods)}个):")
    for method, params, return_type in methods:
        param_str = ", ".join(params)
        print(f"  - {method}({param_str}) -> {return_type}")

    print(f"\n属性 (共{len(properties)}个):")
    for prop, prop_type in properties:
        print(f"  - {prop}: {prop_type}")

    print(f"\n总计: {len(methods) + len(properties)}个成员")
    print(f"方法: {len(methods)}个")
    print(f"属性: {len(properties)}个")

    try:
        obj.Quit()
    except:
        pass

# 使用示例
analyze_com_object("Excel.Application")