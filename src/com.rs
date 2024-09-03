use std::{collections::HashMap, ptr};
use windows::core::{BSTR, VARIANT};
use windows::Win32::System::Com::{
    ITypeInfo, COINIT_MULTITHREADED, DISPATCH_PROPERTYGET, DISPATCH_PROPERTYPUT, DISPPARAMS,
    FUNCDESC, TYPEDESC, VARDESC,
};
use windows::Win32::System::Ole::LoadTypeLib;
use windows::Win32::System::Variant::{
    VT_BOOL, VT_BSTR, VT_CARRAY, VT_CY, VT_DATE, VT_DECIMAL, VT_DISPATCH, VT_EMPTY, VT_ERROR,
    VT_HRESULT, VT_I1, VT_I2, VT_I4, VT_I8, VT_INT, VT_INT_PTR, VT_LPSTR, VT_LPWSTR, VT_NULL,
    VT_PTR, VT_R4, VT_R8, VT_RECORD, VT_SAFEARRAY, VT_UI1, VT_UI2, VT_UI4, VT_UI8, VT_UINT,
    VT_UINT_PTR, VT_UNKNOWN, VT_USERDEFINED, VT_VARIANT, VT_VOID,
};
use windows::{
    core::{Interface, Result, GUID, HSTRING},
    Win32::System::Com::{
        CLSIDFromProgID, CoCreateInstance, CoInitializeEx, CoUninitialize, IDispatch,
        CLSCTX_LOCAL_SERVER,
    },
};

unsafe fn get_params(func_desc: *const FUNCDESC, type_info: &ITypeInfo) -> String {
    let param_count = (*func_desc).cParams as usize;
    let mut params = Vec::with_capacity(param_count);
    let mut names = vec![BSTR::default(); param_count + 1];
    let mut pc_names: u32 = 0;

    let _ = type_info.GetNames((*func_desc).memid, names.as_mut_slice(), &mut pc_names);

    for i in 0..param_count {
        let param = &(*func_desc).lprgelemdescParam.offset(i as isize);
        let param_type = get_rust_type(&(*(*param)).tdesc);
        let param_name = if (i + 1) < pc_names as usize {
            names[i + 1].to_string()
        } else {
            format!("param{}", i)
        };
        params.push(format!("{}: {}", param_name, param_type));
    }

    params.join(", ")
}

#[allow(non_snake_case)]
fn get_rust_type(tdesc: &TYPEDESC) -> String {
    match tdesc.vt {
        VT_EMPTY => "()".to_string(),
        VT_VOID => "()".to_string(),
        VT_NULL => "Option<()>".to_string(),
        VT_I2 => "i16".to_string(),
        VT_I4 => "i32".to_string(),
        VT_R4 => "f32".to_string(),
        VT_R8 => "f64".to_string(),
        VT_CY => "Currency".to_string(),
        VT_DATE => "Date".to_string(),
        VT_BSTR => "String".to_string(),
        VT_DISPATCH => "IDispatch".to_string(),
        VT_ERROR => "Error".to_string(),
        VT_BOOL => "bool".to_string(),
        VT_VARIANT => "Variant".to_string(),
        VT_UNKNOWN => "IUnknown".to_string(),
        VT_DECIMAL => "Decimal".to_string(),
        VT_I1 => "i8".to_string(),
        VT_UI1 => "u8".to_string(),
        VT_UI2 => "u16".to_string(),
        VT_UI4 => "u32".to_string(),
        VT_I8 => "i64".to_string(),
        VT_UI8 => "u64".to_string(),
        VT_INT => "isize".to_string(),
        VT_UINT => "usize".to_string(),
        VT_HRESULT => "HRESULT".to_string(),
        VT_PTR => "Pointer".to_string(),
        VT_SAFEARRAY => "SafeArray".to_string(),
        VT_USERDEFINED => "UserDefined".to_string(),
        VT_LPSTR => "*mut i8".to_string(),
        VT_LPWSTR => "*mut u16".to_string(),
        VT_RECORD => "Record".to_string(),
        VT_CARRAY => "CArray".to_string(),
        VT_INT_PTR => "*mut isize".to_string(),
        VT_UINT_PTR => "*mut usize".to_string(),
        _ => format!("Unknown({})", tdesc.vt.0),
    }
}

trait ComObject {
    fn new_com(prog_id: &str) -> Result<Self>
    where
        Self: Sized;
    fn get_dispatch(&self) -> &IDispatch;
    fn get_disp_id_cache(&self) -> &HashMap<String, i32>;
    fn init_disp_id_cache(&mut self) -> Result<()>;
    fn get_disp_id(&self, name: &str) -> Result<i32> {
        if let Some(&id) = self.get_disp_id_cache().get(name) {
            Ok(id)
        } else {
            panic!("Failed to get dispatch id for property: {name}"); // TODO： use anyhow result
        }
    }

    fn set_property<T>(&self, name: &str, value: T) -> Result<()>
    where
        T: Into<VARIANT>,
    {
        let id = self.get_disp_id(name)?;
        let mut args = [value.into()];
        let mut params = DISPPARAMS {
            rgvarg: args.as_mut_ptr(),
            rgdispidNamedArgs: ptr::null_mut(),
            cArgs: 1,
            cNamedArgs: 0,
        };

        unsafe {
            self.get_dispatch().Invoke(
                id,
                ptr::null(),
                0,
                DISPATCH_PROPERTYPUT,
                &mut params,
                None,
                None,
                None,
            )
        }
    }

    fn get_property(&self, name: &str) -> Result<IDispatch> {
        let id = self.get_disp_id(name)?;
        let mut result = VARIANT::default();
        let params = DISPPARAMS {
            rgvarg: std::ptr::null_mut(),
            rgdispidNamedArgs: std::ptr::null_mut(),
            cArgs: 0,
            cNamedArgs: 0,
        };

        unsafe {
            self.get_dispatch().Invoke(
                id,
                &GUID::zeroed(),
                0,
                DISPATCH_PROPERTYGET,
                &params,
                Some(&mut result),
                None,
                None,
            )?;
            if result.as_raw().Anonymous.Anonymous.vt == VT_DISPATCH.0 {
                let pdispatch = result.as_raw().Anonymous.Anonymous.Anonymous.pdispVal;
                if !pdispatch.is_null() {
                    Ok(IDispatch::from_raw(pdispatch))
                } else {
                    panic!("属性是空的 IDispatch 指针");
                }
            } else {
                panic!(
                    "属性不是 IDispatch 类型, 而是 {:?} 类型",
                    result.as_raw().Anonymous.Anonymous.vt
                );
            }
        }
    }

    fn get_all_methods_and_fields(&self) -> Result<HashMap<String, i32>> {
        let mut cache = HashMap::new();
        unsafe {
            let type_info = self
                .get_dispatch()
                .GetTypeInfo(0, 0)
                .expect("获取类型信息失败");
            let type_attr = type_info.GetTypeAttr().expect("获取类型属性失败");

            println!("## 方法");
            let mut cnt = 0;
            for i in 0..(*type_attr).cFuncs {
                let func_desc = type_info.GetFuncDesc(i.into()).expect("获取函数描述失败");
                let (name, doc, help) = match get_doc(&type_info, DescType::FuncDesc(func_desc)) {
                    Some(value) => value,
                    None => continue,
                };
                let params = get_params(func_desc, &type_info);
                let return_type = get_rust_type(&(*func_desc).elemdescFunc.tdesc);
                cnt += 1;
                println!(
                    "{cnt}: fn {}({}){} {}",
                    name,
                    params,
                    if return_type == "()" {
                        String::new()
                    } else {
                        format!(" -> {return_type}")
                    },
                    if !doc.is_empty() || !help.is_empty() {
                        format!("`{}`\n{}", doc, help)
                    } else {
                        String::new()
                    }
                );
                cache.insert(name, (*func_desc).memid);
                type_info.ReleaseFuncDesc(func_desc);
            }

            println!("\n## 属性");
            cnt = 0;
            for i in 0..(*type_attr).cVars {
                let var_desc = type_info.GetVarDesc(i.into()).expect("获取变量描述失败");
                let (name, doc, help) = match get_doc(&type_info, DescType::VarDesc(var_desc)) {
                    Some(value) => value,
                    None => continue,
                };
                cnt += 1;
                println!(
                    "{cnt}: {name} {}",
                    if !doc.is_empty() || !help.is_empty() {
                        format!("`{}`\n{}", doc, help)
                    } else {
                        String::new()
                    }
                );
                cache.insert(name, (*var_desc).memid);
                type_info.ReleaseVarDesc(var_desc);
            }

            type_info.ReleaseTypeAttr(type_attr);
        }
        Ok(cache)
    }
}

enum DescType {
    FuncDesc(*mut FUNCDESC),
    VarDesc(*mut VARDESC),
}

fn get_doc(type_info: &ITypeInfo, desc: DescType) -> Option<(String, String, String)> {
    let memid = match desc {
        DescType::FuncDesc(func_desc) => unsafe { (*func_desc).memid },
        DescType::VarDesc(var_desc) => unsafe { (*var_desc).memid },
    };

    let mut name_bstr = BSTR::default();
    let mut doc_string_bstr = BSTR::default();
    let mut help_context = 0u32;
    let mut help_file_bstr = BSTR::default();

    let r = unsafe {
        type_info.GetDocumentation(
            memid,
            Some(&mut name_bstr),
            Some(&mut doc_string_bstr),
            &mut help_context,
            Some(&mut help_file_bstr),
        )
    };

    if r.is_err() || name_bstr.is_empty() {
        return None;
    }

    let final_name = name_bstr.to_string();
    let final_doc = doc_string_bstr.to_string();
    let final_help = if !help_file_bstr.is_empty() {
        format!(
            "Help file: {}, Context: {}",
            help_file_bstr.to_string(),
            help_context
        )
    } else {
        String::new()
    };

    Some((final_name, final_doc, final_help))
}

struct ExcelApplication {
    dispatch: IDispatch,
    disp_id_cache: HashMap<String, i32>,
}

impl ComObject for ExcelApplication {
    fn new_com(prog_id: &str) -> Result<Self> {
        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED).unwrap();
            let mut excel = Self {
                dispatch: CoCreateInstance(
                    &CLSIDFromProgID(&HSTRING::from(prog_id))?,
                    None,
                    CLSCTX_LOCAL_SERVER,
                )?,
                disp_id_cache: HashMap::new(),
            };

            excel.init_disp_id_cache()?;
            Ok(excel)
        }
    }

    fn get_dispatch(&self) -> &IDispatch {
        &self.dispatch
    }

    fn get_disp_id_cache(&self) -> &HashMap<String, i32> {
        &self.disp_id_cache
    }

    fn init_disp_id_cache(&mut self) -> Result<()> {
        self.disp_id_cache = self.get_all_methods_and_fields()?;
        Ok(())
    }
}

impl ExcelApplication {
    pub fn new(visible: bool, alert: bool) -> Result<Self> {
        let excel = Self::new_com("Excel.Application")?;
        excel.get_work_books()?;
        excel.get_sheets()?;
        // excel.set_visible(visible)?;
        // excel.set_alert(alert)?;
        // excel.get_sheets()?;

        Ok(excel)
    }

    fn set_visible(&self, visible: bool) -> Result<()> {
        self.set_property("Visible", visible)
    }

    fn set_alert(&self, alert: bool) -> Result<()> {
        self.set_property("DisplayAlerts", alert)
    }

    fn get_work_books(&self) -> Result<()> {
        self.get_property("Workbooks")?;
        Ok(())
    }
    fn get_sheets(&self) -> Result<()> {
        self.get_property("Worksheets")?;
        Ok(())
    }
}

impl Drop for ExcelApplication {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}

pub fn test() -> Result<()> {
    // Unhandled exception at 0x00007FF854F73900 (oleaut32.dll) in test_rust.exe: 0xC0000005: Access violation reading location 0x0000000000000000.
    let excel = ExcelApplication::new(true, true)?;
    Ok(())
}
