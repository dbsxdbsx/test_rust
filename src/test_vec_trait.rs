use std::collections::HashSet;

pub(crate) trait ExtraTraitForVec<T: Clone + Eq + std::hash::Hash> {
    /// This function finds unique and duplicate elements in a vector.
    ///
    /// # Arguments
    ///
    /// * `exclude_duplicates_in_uniques` - A boolean that indicates whether to exclude duplicates in the unique elements.
    /// * `exclude_duplicates_in_duplicates` - A boolean that indicates whether to exclude duplicates in the duplicate elements.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use crate::utils::misc::ExtraTraitForVec;
    ///
    /// let vec = vec![1, 2, 3, 4, 5, 1, 2, 3];
    /// let (uniques, duplicates) = vec.find_uniques_and_duplicates(true, true);
    ///
    /// assert_eq!(uniques, vec![4, 5]);
    /// assert_eq!(duplicates, vec![1, 2, 3]);
    ///
    /// let (uniques, duplicates) = vec.find_uniques_and_duplicates(false, false);
    ///
    /// assert_eq!(uniques, vec![1, 2, 3, 4, 5]);
    /// assert_eq!(duplicates, vec![1, 2, 3]);
    /// ```
    fn find_uniques_and_duplicates(
        &self,
        exclude_duplicates_in_uniques: bool,
        exclude_duplicates_in_duplicates: bool,
    ) -> (Vec<T>, Vec<T>);

    fn find_uniques(&self) -> Vec<T>;

    fn find_duplicates(&self, exclude_multi_duplicates: bool) -> Vec<T>;
}

impl<T: Clone + Eq + std::hash::Hash> ExtraTraitForVec<T> for Vec<T> {
    fn find_uniques_and_duplicates(
        &self,
        exclude_duplicates_in_uniques: bool,
        exclude_duplicates_in_duplicates: bool,
    ) -> (Vec<T>, Vec<T>) {
        let mut uniques = Vec::new();
        let mut duplicates = Vec::new();
        let mut seen = HashSet::new();

        for item in self {
            if !seen.insert(item.clone()) {
                duplicates.push(item.clone());
            } else {
                uniques.push(item.clone());
            }
        }

        if exclude_duplicates_in_uniques {
            uniques.retain(|item| !duplicates.contains(item));
        }

        if exclude_duplicates_in_duplicates {
            let mut seen = HashSet::new();
            duplicates = duplicates
                .iter()
                .filter_map(|x| {
                    if seen.insert(x.clone()) {
                        Some(x.clone())
                    } else {
                        None
                    }
                })
                .collect();
        }
        (uniques, duplicates)
    }

    fn find_uniques(&self) -> Vec<T> {
        let (uniques, _) = self.find_uniques_and_duplicates(false, false);
        uniques
    }

    fn find_duplicates(&self, exclude_multi_duplicates: bool) -> Vec<T> {
        let (_, duplicates) = self.find_uniques_and_duplicates(true, exclude_multi_duplicates);
        duplicates
    }
}
