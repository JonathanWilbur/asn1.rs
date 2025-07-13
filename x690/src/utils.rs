

#[cfg(feature = "likely_stable")]
#[inline]
pub(crate) fn likely (expr: bool) -> bool {
    likely_stable::likely(expr)
}

#[cfg(feature = "likely_stable")]
#[inline]
pub(crate) fn unlikely (expr: bool) -> bool {
    likely_stable::unlikely(expr)
}

#[cfg(not(feature = "likely_stable"))]
#[inline]
pub(crate) fn likely (expr: bool) -> bool {
    expr
}

#[cfg(not(feature = "likely_stable"))]
#[inline]
pub(crate) fn unlikely (expr: bool) -> bool {
    expr
}
