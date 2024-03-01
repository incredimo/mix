# mx  
markdown with extra 🚀 firepower!

## What is it
mx is a markdown renderer that extends the original markdown syntax with a few extra features.  
It makes markdown more powerful and expressive, while still being compatible with the original markdown syntax.

# Markdown
as expected, mx supports the original markdown syntax and all its features. it will render everything as expected. but additionally, it can generate export with custom css and js.

---
## Basic Syntax

### Headings
```markdown
# Heading 1
## Heading 2
### Heading 3
#### Heading 4
##### Heading 5
###### Heading 6
```

Here is a heading: `# Heading`, **don't do this:** `#Heading` 

### Emphasis
```markdown
Emphasis, aka italics, with *asterisks* or _underscores_.

Strong emphasis, aka bold, with **asterisks** or __underscores__.

Combined emphasis with **asterisks and _underscores_**.

Strikethrough uses two tildes. ~~Scratch this.~~
```

### Line Breaks
```markdown
First line with two spaces after.  
And the next line.
```

### Lists

#### Ordered Lists
```markdown
1. First item
2. Second item
3. Third item
```

#### Unordered Lists
```markdown
- First item
- Second item
- Third item
```

### Links
```markdown
Link with text: [link-text](https://www.google.com)
```

### Images
```markdown
Image with alt text: ![alt-text](https://camo.githubusercontent.com/4d89cd791580bfb19080f8b0844ba7e1235aa4becc3f43dfd708a769e257d8de/68747470733a2f2f636e642d70726f642d312e73332e75732d776573742d3030342e6261636b626c617a6562322e636f6d2f6e65772d62616e6e6572342d7363616c65642d666f722d6769746875622e6a7067)

Image without alt text: ![](https://camo.githubusercontent.com/4d89cd791580bfb19080f8b0844ba7e1235aa4becc3f43dfd708a769e257d8de/68747470733a2f2f636e642d70726f642d312e73332e75732d776573742d3030342e6261636b626c617a6562322e636f6d2f6e65772d62616e6e6572342d7363616c65642d666f722d6769746875622e6a7067)
```

### Code Blocks

#### Inline Code Block
```markdown
Inline `code` has `back-ticks around` it.
```

#### Blocks of Code
<pre>
```javascript
var s = "JavaScript syntax highlighting";
alert(s);
```
 
```python
s = "Python syntax highlighting"
print s
```
 
```
No language indicated, so no syntax highlighting. 
But let's throw in a <b>tag</b>.
```
</pre>

### Tables

There must be at least 3 dashes separating each header cell.
The outer pipes (|) are optional, and you don't need to make the raw Markdown line up prettily.

```markdown
| Heading 1 | Heading 2 | Heading 3 |
|---|---|---|
| col1 | col2 | col3 |
| col1 | col2 | col3 |
```

### Task list

To create a task list start line with square brackets with an empty space.
Ex: [ <space> ] and add text for task.
To check the task replace the space between the bracket with "x".

```markdown
[x] Write the post
[ ] Update the website
[ ] Contact the user
```

## Reference

Link: [markdown guide](https://www.markdownguide.org/cheat-sheet)

## Extra Features

on top of the original markdown syntax, mx supports a few extra features that make it more powerful and expressive.
let's see them in action.

### Custom CSS and JS
you can add custom css and js to the markdown file by adding them right in the markdown file.
for example, you can add a custom css file like this:

```css [mx:css]
/* custom.css */
body {
  background-color: #f0f0f0;
}

h1 {
  color: red;
}
```
a usual mardown renderer will just display this as a code block, but mx will intelligently exclude the codeblock from output and instead include the css as a header level style.

### Custom HTML 
you can add custom html to the markdown file by adding them right in the markdown file.
for example, you can add a custom html like this:

<input type="text" placeholder="custom html" style="width: 100%; padding: 10px; margin: 10px 0;"/>

