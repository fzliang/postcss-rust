use rs_nanoid;

#[derive(Debug)]
pub struct FilePosition<'a> {
    /**
     * URL for the source file.
     */
    url: &'a str,

    /**
     * Absolute path to the source file.
     */
    file: Option<&'a str>,

    /**
     * Line of inclusive start position in source file.
     */
    line: u32,

    /**
     * Column of inclusive start position in source file.
     */
    column: u32,

    /**
     * Line of exclusive end position in source file.
     */
    end_line: Option<u32>,

    /**
     * Column of exclusive end position in source file.
     */
    end_column: Option<u32>,

    /**
     * Source code.
     */
    source: Option<&'a str>,
}

#[derive(Debug)]
pub struct Input<'a> {
    /**
     * Input CSS source.
     *
     * ```js
     * const input = postcss.parse('a{}', { from: file }).input
     * input.css //=> "a{}"
     * ```
     */
    pub css: &'a str,

    /**
     * The absolute path to the CSS source file defined
     * with the `from` option.
     *
     * ```js
     * const root = postcss.parse(css, { from: 'a.css' })
     * root.source.input.file //=> '/home/ai/a.css'
     * ```
     */
    pub file: Option<&'a str>,

    /**
     * The unique ID of the CSS source. It will be created if `from` option
     * is not provided (because PostCSS does not know the file path).
     *
     * ```js
     * const root = postcss.parse(css)
     * root.source.input.file //=> undefined
     * root.source.input.id   //=> "<input css 8LZeVF>"
     * ```
     */
    pub id: Option<String>,
}

impl<'a> Input<'a> {
    pub fn new(css: &'a str) -> Self {
        let nanoid = rs_nanoid::standard_unsecure::<6>().to_string();

        Self {
            css,
            file: None,
            id: Some(format!("<input css {nanoid}>")),
        }
    }
}
