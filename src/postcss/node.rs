use std::rc::Weak;

use super::{input::Input, node_type::NodeType};

#[derive(Debug)]
pub struct Position {
    /**
     * Source offset in file. It starts from 0.
     */
    pub offset: u32,

    /**
     * Source line in file. In contrast to `offset` it starts from 1.
     */
    pub column: u32,

    /**
     * Source column in file.
     */
    pub line: u32,
}

impl Position {
    pub fn new(offset: u32, column: u32, line: u32) -> Self {
        Self {
            offset,
            column,
            line,
        }
    }
}

#[derive(Debug)]
pub struct Range {
    /**
     * Start position, inclusive.
     */
    pub start: Position,

    /**
     * End position, exclusive.
     */
    pub end: Position,
}

#[derive(Debug)]
pub struct Source<'a> {
    /**
     * The file source of the node.
     */
    pub input: &'a Input<'a>,

    /**
     * The inclusive starting position of the node’s source.
     */
    pub start: Position,

    /**
     * The inclusive ending position of the node's source.
     */
    pub end: Option<Position>,
}

impl<'a> Source<'a> {
    pub fn new(input: &'a Input<'a>, start: Position, end: Option<Position>) -> Self {
        Self { input, start, end }
    }
}

#[derive(Debug)]
pub struct Raws<'a> {
    /**
     * `before`: the space symbols before the node. It also stores `*`
     *  and `_` symbols before the declaration (IE hack).
     */
    pub before: Option<&'a str>,

    /**
     *  `after`: the space symbols after the last child of the node
     *  to the end of the node.
     */
    pub after: Option<&'a str>,

    /**
     * `between`: the symbols between the property and value
     *  for declarations, selector and `{` for rules, or last parameter
     *  and `{` for at-rules.
     */
    pub between: Option<&'a str>,

    /**
     * `semicolon`: contains true if the last child has
     *  an (optional) semicolon.
     */
    pub semicolon: Option<&'a str>,

    /*
     * `afterName`: the space between the at-rule name and its parameters.
     */
    pub after_name: Option<&'a str>,

    // `left`: the space symbols between `/*` and the comment’s text.
    pub left: Option<&'a str>,

    /**
     * `right`: the space symbols between the comment’s text
     *  and <code>*&#47;</code>.
     */
    pub right: Option<&'a str>,

    /**
     * `important`: the content of the important statement,
     *  if it is not just `!important`.
     */
    pub important: Option<&'a str>,
}

#[derive(Debug)]
pub struct Node<'a> {
    /**
     * tring representing the node’s type. Possible values are `root`, `atrule`,
     * `rule`, `decl`, or `comment`.
     *
     * ```js
     * new Declaration({ prop: 'color', value: 'black' }).type //=> 'decl'
     * ```
     */
    pub node_type: &'a str,

    /**
     * The node’s parent node.
     *
     * ```js
     * root.nodes[0].parent === root
     * ```
     */
    parent: Option<Weak<Box<NodeType<'a>>>>,

    /**
     * The input source of the node.
     *
     * The property is used in source map generation.
     *
     * If you create a node manually (e.g., with `postcss.decl()`),
     * that node will not have a `source` property and will be absent
     * from the source map. For this reason, the plugin developer should
     * consider cloning nodes to create new ones (in which case the new node’s
     * source will reference the original, cloned node) or setting
     * the `source` property manually.
     *
     * ```js
     * decl.source.input.from //=> '/home/ai/a.sass'
     * decl.source.start      //=> { line: 10, column: 2 }
     * decl.source.end        //=> { line: 10, column: 12 }
     * ```
     *
     * ```js
     * // Bad
     * const prefixed = postcss.decl({
     *   prop: '-moz-' + decl.prop,
     *   value: decl.value
     * })
     *
     * // Good
     * const prefixed = decl.clone({ prop: '-moz-' + decl.prop })
     * ```
     *
     * ```js
     * if (atrule.name === 'add-link') {
     *   const rule = postcss.rule({ selector: 'a', source: atrule.source })
     *   atrule.parent.insertBefore(atrule, rule)
     * }
     * ```
     */
    pub source: Option<Source<'a>>,
}

impl<'a> Node<'a> {
    pub fn new(node_type: &'a str) -> Self {
        Self {
            node_type,
            source: None,
            parent: None,
        }
    }
}
