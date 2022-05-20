use std::collections::HashMap;

pub struct Params {
    keys: Vec<String>,
    values: Vec<Vec<String>>,
}

impl Params {
    pub fn new<K, V>(keys: K, values: V) -> Self
    where
        K: IntoIterator,
        V: IntoIterator,
        K::Item: ToString,
        V::Item: IntoIterator,
        <V::Item as IntoIterator>::Item: ToString,
    {
        let keys: Vec<String> = keys.into_iter().map(|k| k.to_string()).collect();
        let values: Vec<Vec<String>> = values
            .into_iter()
            .map(|list| list.into_iter().map(|v| v.to_string()).collect::<Vec<_>>())
            .collect();

        if keys.len() != values.len() {
            //FIXME: fix to return an error instead of occuring a panic.
            panic!("The length of keys must be equal to that of values.");
        }

        Self { keys, values }
    }

    pub fn get_combination(&self) -> usize {
        self.values.iter().map(|v| v.len()).product::<_>()
    }

    pub fn iter(&self) -> ParamsIter {
        ParamsIter::new(self)
    }
}

pub struct ParamsIter<'a> {
    params: &'a Params,
    idx_comb_iter: IndexCombIter,
}

impl<'a> ParamsIter<'a> {
    fn new(params: &'a Params) -> Self {
        let idx_len = params.values.iter().map(|v| v.len()).collect();

        Self {
            params,
            idx_comb_iter: IndexCombIter::new(idx_len),
        }
    }
}

impl<'a> Iterator for ParamsIter<'a> {
    //TODO: fix Item from HashMap<String, String> to HashMap<&str, &str> to avoid duplication of keys and values
    type Item = HashMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let idx_list = self.idx_comb_iter.next()?;

        let mut map = HashMap::new();

        for (i, idx) in idx_list.iter().enumerate() {
            map.insert(
                self.params.keys[i].to_string(),
                self.params.values[i][*idx].to_string(),
            );
        }

        Some(map)
    }
}

struct IndexCombIter {
    idx_len: Vec<usize>,
    cur_idx: Option<Vec<usize>>,
}

impl IndexCombIter {
    fn new(idx_len: Vec<usize>) -> Self {
        for len in &idx_len {
            if len <= &0 {
                //FIXME: fix to return an error not to panic here.
                panic!("Each element in idx_len must be greater than 0.");
            }
        }

        Self {
            idx_len,
            cur_idx: None,
        }
    }

    fn init(&mut self) {
        let cur_idx = vec![0; self.idx_len.len()];

        self.cur_idx = Some(cur_idx);
    }
}

impl<'a> Iterator for IndexCombIter {
    //TODO: add a life parameter and fix the Item from Vec<i32> to &'a Vec<i32>
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cur_idx {
            None => {
                self.init();
            }
            Some(ref mut cur_idx) => {
                while !cur_idx.is_empty() {
                    let i = cur_idx.len() - 1;
                    let next = cur_idx.pop()? + 1;

                    if next < self.idx_len[i] {
                        cur_idx.push(next);
                        break;
                    }
                }

                if cur_idx.is_empty() {
                    return None;
                }

                while cur_idx.len() < self.idx_len.len() {
                    cur_idx.push(0);
                }
            }
        }

        Some(
            self.cur_idx
                .as_ref()
                .expect("cur_idx must have some value")
                .clone(),
        )
    }
}

//TODO: add tests about ParamsIter
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_combination_with_one_key() {
        let keys = vec!["N"];
        let values = vec![vec!["1", "10", "100"]];
        let params = Params::new(keys, values);

        assert_eq!(3, params.get_combination());
    }

    #[test]
    fn get_combination_with_two_keys() {
        let keys = vec!["N", "MAX_NODE"];
        let values = vec![vec![1, 10, 100], vec![1, 2]];

        let params = Params::new(keys, values);

        assert_eq!(6, params.get_combination());
    }

    #[test]
    fn params_iter_with_one_key() {
        let keys = vec!["N"];
        let values = vec![vec!["1", "10", "100"]];
        let params = Params::new(keys, values);
        let mut iter = params.iter();

        assert_eq!(
            Some(HashMap::from([("N".to_string(), "1".to_string())])),
            iter.next()
        );
        assert_eq!(
            Some(HashMap::from([("N".to_string(), "10".to_string())])),
            iter.next()
        );
        assert_eq!(
            Some(HashMap::from([("N".to_string(), "100".to_string())])),
            iter.next()
        );
        assert_eq!(None, iter.next());
    }

    #[test]
    fn params_iter_with_two_keys() {
        let keys = vec!["N", "MAX_NODE"];
        let values = vec![vec![1, 10, 100], vec![1, 2]];

        let params = Params::new(keys, values);
        let mut iter = params.iter();

        assert_eq!(
            Some(HashMap::from([
                ("N".to_string(), "1".to_string()),
                ("MAX_NODE".to_string(), "1".to_string())
            ])),
            iter.next()
        );
        assert_eq!(
            Some(HashMap::from([
                ("N".to_string(), "1".to_string()),
                ("MAX_NODE".to_string(), "2".to_string())
            ])),
            iter.next()
        );

        assert_eq!(
            Some(HashMap::from([
                ("N".to_string(), "10".to_string()),
                ("MAX_NODE".to_string(), "1".to_string())
            ])),
            iter.next()
        );
        assert_eq!(
            Some(HashMap::from([
                ("N".to_string(), "10".to_string()),
                ("MAX_NODE".to_string(), "2".to_string())
            ])),
            iter.next()
        );

        assert_eq!(
            Some(HashMap::from([
                ("N".to_string(), "100".to_string()),
                ("MAX_NODE".to_string(), "1".to_string())
            ])),
            iter.next()
        );
        assert_eq!(
            Some(HashMap::from([
                ("N".to_string(), "100".to_string()),
                ("MAX_NODE".to_string(), "2".to_string())
            ])),
            iter.next()
        );

        assert_eq!(None, iter.next());
    }

    #[test]
    fn index_comb_iter_with_one_vec() {
        let idx_len = vec![4];
        let mut iter = IndexCombIter::new(idx_len);

        assert_eq!(Some(vec![0]), iter.next());
        assert_eq!(Some(vec![1]), iter.next());
        assert_eq!(Some(vec![2]), iter.next());
        assert_eq!(Some(vec![3]), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn index_comb_iter_with_two_vecs() {
        let idx_len = vec![3, 2];
        let mut iter = IndexCombIter::new(idx_len);

        assert_eq!(Some(vec![0, 0]), iter.next());
        assert_eq!(Some(vec![0, 1]), iter.next());
        assert_eq!(Some(vec![1, 0]), iter.next());
        assert_eq!(Some(vec![1, 1]), iter.next());
        assert_eq!(Some(vec![2, 0]), iter.next());
        assert_eq!(Some(vec![2, 1]), iter.next());

        assert_eq!(None, iter.next());
    }

    #[test]
    fn index_comb_iter_with_three_vecs() {
        let idx_len = vec![2, 1, 4];
        let mut iter = IndexCombIter::new(idx_len);

        assert_eq!(Some(vec![0, 0, 0]), iter.next());
        assert_eq!(Some(vec![0, 0, 1]), iter.next());
        assert_eq!(Some(vec![0, 0, 2]), iter.next());
        assert_eq!(Some(vec![0, 0, 3]), iter.next());
        assert_eq!(Some(vec![1, 0, 0]), iter.next());
        assert_eq!(Some(vec![1, 0, 1]), iter.next());
        assert_eq!(Some(vec![1, 0, 2]), iter.next());
        assert_eq!(Some(vec![1, 0, 3]), iter.next());

        assert_eq!(None, iter.next());
    }
}
