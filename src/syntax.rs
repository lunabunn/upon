//! Documents the template syntax.
//!
//! An `upon` template is simply a piece of UTF-8 text. It can be embedded in
//! the binary or provided at runtime (e.g. read from a file). A template
//! contains [**expressions**](#expressions) for rendering values and
//! [**blocks**](#blocks) for controlling logic. These require you to use
//! specific syntax delimiters in the template. Because `upon` allows you to
//! configure these delimiters, this document will only refer to the
//! [**default**][crate::Syntax::default] configuration.
//!
//! # Expressions
//!
//! Expressions are available everywhere in templates and they can be emitted by
//! wrapping them in `{{ ... }}`.
//!
//! ## Literals
//!
//! The simplest form of expressions are literals. The following types are
//! available in templates.
//!
//! - Booleans: `true`, `false`
//! - Integers: `42`, `0o52`, `-0x2a`
//! - Floats: `0.123`, `-3.14`, `5.23e10`
//! - Strings: `"Hello World!"`, escape characters are supported: `\r`, `\n`,
//!   `\t`, `\\`, `\"`
//!
//! ## Values
//!
//! You can lookup up existing values in the current scope by name. The
//! following would lookup the field "name" in the current scope and insert it
//! into the rendered output.
//!
//! ```text
//! Hello {{ name }}!
//! ```
//!
//! You can access nested fields using a dotted path. The following would first
//! lookup the field "user" and then lookup the field "name" within it.
//!
//! ```text
//! Hello {{ user.name }}!
//! ```
//!
//! You can also use this syntax to lookup a particular index of a list. For
//! each of the expressions in the following code, first the field "users" is
//! looked up, then a particular user is selected from the list by index.
//! Finally, the field "name" is looked up from the selected user.
//!
//! ```text
//! Hello {{ users.0.name }}!
//! And hello {{ users.1.name }}!
//! And also hello {{ users.2.name }}!
//! ```
//!
//! ## Filters
//!
//! Filters can be applied to existing expressions using the `|` (pipe)
//! operator. The simplest filters take no extra arguments and are just
//! specified by name. For example, assuming a filter called `lower` is
//! registered in the engine the following would produce an expression with the
//! `user.name` value transformed to lowercase.
//!
//! ```html
//! {{ user.name | lower }}
//! ```
//!
//! Filters can also take arguments which must be a sequence of comma separated
//! values or literals. In the following we lookup the value `page.path` and
//! append a suffix to it.
//!
//! ```html
//! {{ page.path | append: ".html" }}
//! ```
//!
//! See the [`filters`][crate::filters] module documentation for more
//! information on filters.
//!
//! # Blocks
//!
//! Blocks are marked with an opening `{% ... %}` and a closing `{% ... %}`.
//!
//! ## Conditionals
//!
//! Conditionals are marked using an opening `if` block and a closing `endif`
//! block. It can also have zero or more optional `else if` clauses and an
//! optional `else` clause. A conditional renders the contents of the block
//! based on the specified condition which can be any
//! [**expression**](#expressions) but it must resolve to a boolean value.  A
//! boolean expression can be negated by applying the prefix `not`.
//!
//! Consider the following template. If the nested field `user.is_enabled` is
//! returns `false` then the first paragraph would be rendered. Otherwise if
//! `user.has_permission` returns true then the second paragraph would be
//! rendered. If neither condition is satisfied then the HTML table would be
//! rendered.
//!
//! ```html
//! {% if not user.is_enabled %}
//!     <p>User is disabled</p>
//! {% else if user.has_permission %}
//!     <p>User has insufficient permissions</p>
//! {% else %}
//!     <table>...</table>
//! {% endif %}
//! ```
//!
//! ## Loops
//!
//! Loops are marked using an opening `for` block and a closing `endfor` block.
//! A loop renders the contents of the block once for each item in the specified
//! sequence. This is done by unpacking each item into one or two variables.
//! These variables are added to the scope within the loop block and shadow any
//! variables with the same name in the outer scope. The specified sequence can
//! be any [**expression**](#expressions) but it must resolve to a list or map.
//! Additionally, for lists there must a single loop variable and for maps there
//! must be key and value loop variables.
//!
//! Consider the following template. This would render an HTML paragraph for
//! each user in the list.
//!
//! ```html
//! {% for user in users %}
//!     <p>{{ user.name }}</p>
//! {% endfor %}
//! ```
//!
//! Here is an example where `users` is a map.
//!
//! ```html
//! {% for id, user in users %}
//!     <div>
//!         <p>ID: {{ id }}</p>
//!         <p>Name: {{ user.name }}</p>
//!     </div>
//! {% endfor %}
//! ```
//!
//! Additionally, there are three special values available within loops.
//!
//! - `loop.index`: a zero-based index of the current value in the iterable
//! - `loop.first`: `true` if this is the first iteration of the loop
//! - `loop.last`: `true` if this is the last iteration of the loop
//!
//! ```html
//! <ul>
//! {% for user in users %}
//!     <li>{{ loop.index }}. {{ user.name }}</li>
//! {% endfor %}
//! </ul>
//! ```
//!
//! ## With
//!
//! "With" blocks can be used to create a variable from an
//! [**expression**](#expressions). The variable is only valid within the block
//! and it shadows any outer variables with the same name.
//!
//! ```html
//! {% with user.names | join: " " as fullname %}
//!     Hello {{ fullname }}!
//! {% endwith %}
//! ```
//!
//! ## Include
//!
//! "Include" blocks can be used to render nested templates. The nested template
//! must have been registered in the engine before rendering. For example,
//! assuming a template "footer" has been registered in the engine the following
//! would render the template "footer" in the place of the include block. All
//! variables in the current template will be available to the nested template.
//!
//! ```html
//! <body>
//!     ...
//!
//!     {% include "footer" %}
//!
//! </body>
//! ```
//!
//! You can also include the nested using a specific context. In this case the
//! nested template would not have any access to the current template's
//! variables and `path.to.footer.info` would form the global context for the
//! nested template.
//!
//! ```html
//! <body>
//!     ...
//!
//!     {% include "footer" with path.to.footer.info %}
//!
//! </body>
//! ```
//!
//! Self-referential templates and include cycles are allowed but the maximum
//! include depth is restricted by the engine setting
//! [`set_max_include_depth`][crate::Engine::set_max_include_depth].
//!
//! # Whitespace control
//!
//! If an expression or block includes a hyphen `-` character, like `{{-`,
//! `-}}`, `{%-`, and `-%}` then any whitespace in the template adjacent to the
//! tag will be skipped when the template is rendered.
//!
//! Consider the following template.
//!
//! ```text
//! Hello,
//!     {% if user.is_welcome %} and welcome, {% endif %}
//!     {{ user.name }}!
//! ```
//!
//! This doesn't use any whitespace trimming so it would be rendered something
//! like this:
//! ```text
//! Hello,
//!      and welcome,
//!     John!
//! ```
//!
//! We can add whitespace trimming like this.
//! ```text
//! Hello,
//!     {%- if user.is_welcome %} and welcome, {% endif -%}
//!     {{ user.name }}!
//! ```
//!
//! Now it will be rendered without the newlines or extra spaces.
//! ```text
//! Hello, and welcome, John!
//! ```
