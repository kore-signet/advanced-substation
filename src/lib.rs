use std::{array, marker::PhantomData, str::FromStr};

use models::OptionStr;

pub mod models;

struct FilteredLines<'a> {
    lines: std::str::Lines<'a>,
}

impl<'a> Iterator for FilteredLines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.lines.next() {
                Some(v) if v.starts_with(';') => continue,
                Some(v) => return Some(v),
                None => return None,
            }
        }
    }
}

pub struct SSAParser<'a> {
    pub(crate) lines: FilteredLines<'a>,
}

impl<'data> SSAParser<'data> {
    pub fn new(data: &'data str) -> SSAParser<'data> {
        SSAParser {
            lines: FilteredLines {
                lines: data.lines(),
            },
        }
    }

    pub fn section(&mut self) -> Option<RawSectionIterator<'data, '_>> {
        let title_line = self
            .lines
            .next()
            .map(str::trim)
            .and_then(|v| v.split_once('['))
            .and_then(|(_, r)| r.split_once(']'))
            .map(|(l, _)| l)?;
        Some(RawSectionIterator {
            title: title_line,
            parser: self,
        })
    }
}

pub struct RawSectionIterator<'data, 'borrow> {
    pub title: &'data str,
    pub parser: &'borrow mut SSAParser<'data>,
}

impl<'data, 'borrow> RawSectionIterator<'data, 'borrow> {
    pub fn as_key_value<S: KeyValueSection<'data>>(self) -> Option<S::Output<'data, 'borrow>> {
        S::parse(KeyValueSectionIter::new(self))
    }

    pub fn as_stream_section<const MAX_FIELDS: usize, L: LineItem<'data, MAX_FIELDS>>(
        self,
    ) -> Option<LineStreamSectionIter<'data, 'borrow, MAX_FIELDS, L>> {
        LineStreamSectionIter::start(self)
    }
}

impl<'data, 'borrow> Iterator for RawSectionIterator<'data, 'borrow> {
    type Item = (&'data str, &'data str);

    fn next(&mut self) -> Option<Self::Item> {
        let line = match self.parser.lines.next() {
            Some(v) if v.trim().is_empty() => return None,
            Some(v) => v,
            None => return None,
        };

        let (lhs, rhs) = line.split_once(':')?;

        Some((lhs.trim(), rhs.trim()))
    }
}

pub trait KeyValueSection<'data> {
    type Output<'a, 'b>
    where
        'a: 'b,
        'data: 'b;
    type Fields: FromStr;

    fn parse<'b>(
        source: KeyValueSectionIter<'data, 'b, Self::Fields>,
    ) -> Option<Self::Output<'data, 'b>>;
}

pub struct KeyValueSectionIter<'data, 'borrow, Fields: FromStr> {
    pub title: &'data str,
    pub parser: RawSectionIterator<'data, 'borrow>,
    pub spooks: PhantomData<Fields>,
}

impl<'data, 'borrow, Fields: FromStr> KeyValueSectionIter<'data, 'borrow, Fields> {
    pub fn new(
        v: RawSectionIterator<'data, 'borrow>,
    ) -> KeyValueSectionIter<'data, 'borrow, Fields> {
        Self {
            title: v.title,
            parser: v,
            spooks: PhantomData,
        }
    }
}

impl<'data, 'borrow, Fields: FromStr> Iterator for KeyValueSectionIter<'data, 'borrow, Fields> {
    type Item = (Fields, &'data str);

    fn next(&mut self) -> Option<Self::Item> {
        self.parser
            .find_map(|(k, v)| Fields::from_str(k).ok().map(|k| (k, v)))
    }
}

pub trait LineItem<'data, const MAX_FIELDS: usize> {
    type Fields: FromStr + Default + Copy;
    type Item<'a>
    where
        'a: 'data;

    fn validate_section_name(name: &str) -> bool;

    fn parse_from_fields(
        key: &'data str,
        fields: [(Self::Fields, OptionStr<'data>); MAX_FIELDS],
    ) -> Option<Self::Item<'data>>;
}

pub struct LineStreamSectionIter<
    'data,
    'borrow,
    const MAX_FIELDS: usize,
    L: LineItem<'data, MAX_FIELDS>,
> {
    pub title: &'data str,
    field_order: [L::Fields; MAX_FIELDS],
    inner: RawSectionIterator<'data, 'borrow>,
}

impl<'data, 'borrow, const MAX_FIELDS: usize, L: LineItem<'data, MAX_FIELDS>>
    LineStreamSectionIter<'data, 'borrow, MAX_FIELDS, L>
{
    pub fn start(
        mut inner: RawSectionIterator<'data, 'borrow>,
    ) -> Option<LineStreamSectionIter<'data, 'borrow, MAX_FIELDS, L>> {
        if !L::validate_section_name(inner.title) {
            return None;
        }

        let (format_key, format_line) = inner.next()?;
        if !format_key.eq_ignore_ascii_case("format") {
            return None;
        }

        let mut field_order = [L::Fields::default(); MAX_FIELDS];
        for (idx, field) in format_line.split(',').enumerate() {
            field_order[idx] = L::Fields::from_str(field.trim()).ok()?;
        }

        Some(LineStreamSectionIter {
            title: inner.title,
            field_order,
            inner,
        })
    }
}

impl<'data, 'borrow, const MAX_FIELDS: usize, L: LineItem<'data, MAX_FIELDS>> Iterator
    for LineStreamSectionIter<'data, 'borrow, MAX_FIELDS, L>
{
    type Item = L::Item<'data>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (key, values) = self.inner.next()?;

            let mut fields: [(L::Fields, OptionStr<'data>); MAX_FIELDS] =
                array::from_fn(|idx| (self.field_order[idx], None));
            for (idx, value) in values.splitn(MAX_FIELDS, ',').enumerate() {
                fields[idx].1 = Some(value.trim().into());
            }

            if let Some(v) = L::parse_from_fields(key, fields) {
                return Some(v);
            }
        }
    }
}
