pub mod nom {
    use nom::character::complete::{space1, u32, u64};
    use nom::error::ParseError;
    use nom::multi::separated_list1;
    use nom::{IResult, Parser};

    pub fn separated_triple<I, O1, O2, O3, O4, E: ParseError<I>, F, G, H, J>(
        mut sep: F,
        mut first: G,
        mut second: H,
        mut third: J,
    ) -> impl FnMut(I) -> IResult<I, (O2, O3, O4), E>
    where
        F: Parser<I, O1, E>,
        G: Parser<I, O2, E>,
        H: Parser<I, O3, E>,
        J: Parser<I, O4, E>,
    {
        move |input: I| {
            let (input, o1) = first.parse(input)?;
            let (input, _) = sep.parse(input)?;
            let (input, o2) = second.parse(input)?;
            let (input, _) = sep.parse(input)?;
            third.parse(input).map(|(i, o3)| (i, (o1, o2, o3)))
        }
    }

    pub fn numbers_u32(input: &str) -> IResult<&str, Vec<u32>> {
        separated_list1(space1, u32)(input)
    }

    pub fn numbers_u64(input: &str) -> IResult<&str, Vec<u64>> {
        separated_list1(space1, u64)(input)
    }
}
