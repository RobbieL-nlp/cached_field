#[cfg(test)]
mod tests {
    use cached_field::cached_field;

    struct TestStruct {
        test: Option<u8>,
        test0: Option<u8>,
    }

    impl TestStruct {
        #[cached_field]
        fn test(&self) -> u8 {
            0
        }
        #[cached_field("test0")]
        fn test1(&self) -> u8 {
            3
        }
        #[cached_field(field = "test0")]
        pub fn test0(&self) -> u8 {
            5
        }
    }

    trait cache {
        fn test(&mut self) -> &Vec<u8>;
        fn test0(&mut self) -> &Vec<u8>;
    }
    struct TestStruct0 {
        test_borrow: Option<Vec<u8>>,
    }

    impl cache for TestStruct0 {
        #[cached_field("test_borrow", true)]
        fn test(&self) -> Vec<u8> {
            vec![3, 5]
        }

        #[cached_field(field = "test_borrow", borrow = true)]
        fn test0(&self) -> Vec<u8> {
            vec![3, 5]
        }
    }

    trait GCache<T> {
        fn test(&mut self) -> T;
    }

    struct GTestStruct<T> {
        test_generic: Option<T>,
        default: T,
    }

    impl<T: Copy + ToString> GCache<T> for GTestStruct<T> {
        #[cached_field("test_generic")]
        fn test(&self) -> T {
            self.default
        }
    }

    #[test]
    fn same_name_no_args_struct_impl() {
        assert_eq!(
            TestStruct {
                test: None,
                test0: None,
            }
            .test(),
            0
        );
        let mut obj = TestStruct {
            test: None,
            test0: None,
        };

        assert_eq!(0, obj.test());
    }

    #[test]
    fn diff_name_generic_impl() {
        assert_eq!(
            GTestStruct {
                test_generic: None,
                default: 10,
            }
            .test(),
            10
        );
    }

    #[test]
    fn diff_name_struct_impl() {
        assert_eq!(
            TestStruct {
                test: None,
                test0: None,
            }
            .test1(),
            3
        );
        let mut obj = TestStruct {
            test: None,
            test0: None,
        };

        assert_eq!(3, obj.test1());
    }

    #[test]
    fn diff_name_kv_struct_impl() {
        let mut obj = TestStruct {
            test: None,
            test0: None,
        };

        assert_eq!(5, obj.test0());
    }

    #[test]
    fn different_name_pos_args_trait_impl() {
        assert_eq!(*TestStruct0 { test_borrow: None }.test(), vec![3, 5]);
        let mut obj = TestStruct0 { test_borrow: None };

        assert_eq!(vec![3, 5], *obj.test());
        assert_eq!(vec![3, 5], *obj.test());
    }

    #[test]
    fn different_name_kv_trait_impl() {
        assert_eq!(*TestStruct0 { test_borrow: None }.test(), vec![3, 5]);
        let mut obj = TestStruct0 { test_borrow: None };

        assert_eq!(vec![3, 5], *obj.test0());
        assert_eq!(vec![3, 5], *obj.test());
    }
}
