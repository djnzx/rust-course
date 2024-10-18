// fold-left implementation
fn pack2<A: Clone + PartialEq>(xs: &Vec<A>) -> Vec<(A, usize)> {
    let one = |a: &A| (a.clone(), 1);

    xs.iter()
        .fold(Vec::<(A, usize)>::with_capacity(xs.len()), |mut acc, a| {
            match acc.last() {
                // start
                None => vec![one(a)],
                // continue with the same element
                Some((aa, c)) if a == aa => {
                    acc.last_mut()
                        .unwrap()
                        .1 = c + 1;
                    acc
                }
                // continue with the different element
                Some(_) => {
                    acc.push(one(a));
                    acc
                }
            }
        })
}

// iterator-based implementation
fn pack1<A: Clone + PartialEq>(xs: &Vec<A>) -> Vec<(A, usize)> {
    let mut iter = xs.iter();
    let mut cur: Option<(A, usize)> = None;
    let mut acc = Vec::<(A, usize)>::new();

    loop {
        match (iter.next(), &cur) {
            // finish with empty current
            (None, None) => break acc,
            // finish with non-empty current
            (None, Some((v, count))) => {
                acc.push((v.clone(), *count));
                break acc;
            }
            // start
            (Some(a), None) => cur = Some((a.clone(), 1)),
            // continue with the same element
            (Some(a), Some((v, count))) if a == v => cur = Some((a.clone(), count + 1)),
            // continue with the different element
            (Some(a), Some((v, count))) => {
                acc.push((v.clone(), *count));
                cur = Some((a.clone(), 1));
            }
        }
    }
}

fn unpack<A: Clone + PartialEq>(xs: &Vec<(A, usize)>) -> Vec<A> {
    xs.iter()
        .flat_map(|&(ref a, ref count)| vec![a.clone(); *count])
        .collect()
}

#[test]
fn pack_test() {
    let test_cases: Vec<(&str, Vec<(char, usize)>)> = vec![
        ("", vec![]),
        ("a", vec![('a', 1)]),
        ("aa", vec![('a', 2)]),
        ("ab", vec![('a', 1), ('b', 1)]),
        ("aab", vec![('a', 2), ('b', 1)]),
        ("abb", vec![('a', 1), ('b', 2)]),
        ("abba", vec![('a', 1), ('b', 2), ('a', 1)]),
        (
            "abbcccaabc",
            vec![('a', 1), ('b', 2), ('c', 3), ('a', 2), ('b', 1), ('c', 1)],
        ),
    ];

    test_cases
        .iter()
        .for_each(|(s_unpacked, packed)| {
            let unpacked = s_unpacked
                .chars()
                .collect::<Vec<_>>();

            // pack V1
            let packed2 = pack1(&unpacked);
            assert_eq!(packed2, *packed);

            // pack V2
            let packed2 = pack2(&unpacked);
            assert_eq!(packed2, *packed);

            // unpack
            let unpacked2 = unpack(&packed);
            assert_eq!(unpacked2, unpacked);
        });
}
