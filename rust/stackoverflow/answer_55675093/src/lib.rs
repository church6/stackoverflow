use itertools::Itertools; // 0.10.5
use rand; // 0.8.5

// https://docs.rs/rand/latest/rand/distributions/trait.Distribution.html#method.sample_iter

pub fn rand_data(len: usize) -> String {
    use rand::distributions::{Alphanumeric, Distribution};
    let mut rng = rand::thread_rng();
    //Alphanumeric.sample_iter(&mut rng).take(len).collect()
    // String:
    Alphanumeric
        .sample_iter(&mut rng)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn encode_proc(source: &str) -> String {
    let mut retval = String::new();
    let firstchar = source.chars().next();
    let mut currentchar = match firstchar {
        Some(x) => x,
        None => return retval,
    };
    let mut currentcharcount: u32 = 0;
    for c in source.chars() {
        if c == currentchar {
            currentcharcount += 1;
        } else {
            if currentcharcount > 1 {
                retval.push_str(&currentcharcount.to_string());
            }
            retval.push(currentchar);
            currentchar = c;
            currentcharcount = 1;
        }
    }
    if currentcharcount > 1 {
        retval.push_str(&currentcharcount.to_string());
    }
    retval.push(currentchar);
    retval
}

pub fn encode_iter(data: &str) -> String {
    data.chars()
        .group_by(|&c| c)
        .into_iter()
        .map(|(c, group)| match group.count() {
            1 => c.to_string(),
            n => format!("{}{}", n, c),
        })
        .collect()
}

pub fn encode_slim(data: &str) -> String {
    data.chars()
        .batching(|it| {
            it.next()
                .map(|v| (v, it.take_while_ref(|&v2| v2 == v).count() + 1))
        })
        .format_with("", |(c, count), f| match count {
            1 => f(&c),
            n => f(&format_args!("{}{}", n, c)),
        })
        .to_string()
}

struct RunLength<I> {
    iter: I,
    saved: Option<char>,
}

impl<I> RunLength<I>
where
    I: Iterator<Item = char>,
{
    fn new(mut iter: I) -> Self {
        let saved = iter.next();
        Self { iter, saved }
    }
}

impl<I> Iterator for RunLength<I>
where
    I: Iterator<Item = char>,
{
    type Item = (char, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.saved.take().or_else(|| self.iter.next())?;

        let mut count = 1;
        while let Some(n) = self.iter.next() {
            if n == c {
                count += 1
            } else {
                self.saved = Some(n);
                break;
            }
        }

        Some((c, count))
    }
}

pub fn encode_tiny(data: &str) -> String {
    use std::fmt::Write;

    RunLength::new(data.chars()).fold(String::new(), |mut s, (c, count)| {
        match count {
            1 => s.push(c),
            n => write!(&mut s, "{}{}", n, c).unwrap(),
        }
        s
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn all_the_same() {
        let data = rand_data(1024);

        let a = encode_proc(&data);
        let b = encode_iter(&data);
        let c = encode_slim(&data);
        let d = encode_tiny(&data);

        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(a, d);
    }
}
