pub(crate) fn crop_range<T>(target: T, min_value: T, max_value: T) -> T
where
    T: Ord,
{
    min_value.max(max_value.min(target))
}

pub(crate) fn max<T>(value1: T, value2: T, value3: T) -> T
where
    T: Ord,
{
    value1.max(value2.max(value3))
}

pub(crate) fn min<T>(value1: T, value2: T, value3: T) -> T
where
    T: Ord,
{
    value1.min(value2.min(value3))
}
