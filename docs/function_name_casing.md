# function_name_casing

## What it does

Checks for inconsistent name casing in function declarations.

## Why this is bad

Styling within a codebase (and preferably within the ecosystem) should be consistent.

## Configuration

`use_upper` (default: `false`) - A bool that determines whether the lint enforces starting function declarations with uppercase.

`allow` - A list of keywords where the function_name_casing lint will not throw. For instance, `["Foo"]` will allow you to declare `local function Foo() end`, even if `use_upper` is false. This should only be used in cases where necessary, such as when using a third party library that enforces a certain style inconsistent with the rest of your codebase.

## Example

```lua
-- If `use_upper` is true, this will warn
function component:render() end

-- If `allow` includes "render", this will not warn even if `use_upper` is true
function component:render() end
```
