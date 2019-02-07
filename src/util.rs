//// Diff contains the index and left/right values that are different.
type Diff<'a, 'b, T> = Vec<(usize, Option<&'a T>, Option<&'b T>)>;

// Compute the difference between two slices.
pub fn diff<'a, 'b, T>(left: &'a [T], right: &'b [T]) -> Diff<'a, 'b, T>
    where T: PartialEq
{
    let mut diff = vec![];
    let min = std::cmp::min(left.len(), right.len());
    for ii in 0..min {
        if &left[ii] != &right[ii] {
            diff.push((ii, Some(&left[ii]), Some(&right[ii])));
        }
    }
    if left.len() > right.len() {
        for ii in 0..left.len()-right.len() {
            diff.push((min+ii, Some(&left[min+ii]), None))
        }
    }
    if left.len() < right.len() {
        for ii in 0..right.len()-left.len() {
            diff.push((min+ii, None, Some(&right[min+ii])))
        }
    }
    return diff;
}