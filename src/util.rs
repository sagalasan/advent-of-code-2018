pub trait Solve<T> {
    type Output;
    fn solve(input: T) -> Self::Output;
}
