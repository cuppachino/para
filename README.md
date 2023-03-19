# Another TypeScript path alias resolver

Watch a video example [here](https://youtu.be/jwqb7phaRHg)

## What is it?

This is just a small project I used to quickly pick up Rust.
My goal was to provide a simple, non-destructive, unopinionated solution to using path aliases in TypeScript.

## Why is it a problem?

### Your directory structure:
```
> dist/
⌄ src/
  > features/
  > utils/
  - index.mts
```

### When the `features/` directory grows to hold many sub-modules, importing them from other directories can become... annoying to say the least:

`import xyz from "./../../../../../../../index.mjs"`

We can make this much nicer by giving important directories a path *alias* (hence name). This is a native feature of TypeScript! How fortunate!

```jsonc
/* tsconfig.json */
{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@/*", "./src/*",
      "@xyz/*", "./src/features/xyz/*",
    }
  }
}
```

Which would let us write `import xyz from "@xyz"` or `import xyz from "@/features/xyz"` instead.

### Why can't we *just* do that?

If you follow the steps above, transpile the project with `tsc`, and run the output with `node`, you'll get an error.

```
node:
  "something something something... 
  I DON'T KNOW WHAT THAT FILE IS.
  WHAT IS @xyz?
  HOW DO I IMPORT IT? 
  I THINK I JUST WON'T"
exit 1
```

Take a quick peek at the `dist/` output

```
⌄ dist/
  > features/
  > utils/
  - index.mjs
> src/
```

```js
...require("@xyz")...
```

`tsc` will not resolve production paths for you. THAT's where this `[para]` comes in!

### Usage

`[para]` distinguishes itself from other solutions to the problem by allowing you to retain your current workflows and build tools.

- you can keep your current transpilation/bundling tools
- you don't have to update every path in your project to use an extension *(unless two files are named the same)*
- you don't have to ship any additional run-time code.
- you don't have to use experimental module loaders.

#### Harnessing the power of things you've **already configured**

`[para]` knows how to read your tsconfig and knows exactly where to look for files. The cli needs very little (often ZERO) configuration.

According to [`npmjs`](https://docs.npmjs.com/cli/v9/using-npm/scripts#pre--post-scripts), you can give any script an *automatic* before/after step by extending your `package.json` with a brother script of the same name, but prefixed.

```jsonc
/* package.json */
{
  "scripts": {
    "build": "tsc -p ./tsconfig",
    "postbuild": "[para]"
  }
}
```

Assuming your `tsconfig.json` is in the same directory as your `package.json`, that's all you need.
If you're in a monorepo you may wish to to specify a list of tsconfigs.
The exclusion patterns (comma-delimited) may also be specified or extended. You can run `[para] --help` or `[para] -h` for more command details.
