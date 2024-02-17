## Building Wasm Module:

`wasm-pack build --release --target web`

make sure you install

Ensure Rust is installed. You can download and install it from *[Rust](https://www.rust-lang.org/)*.

Install Wasm-pack, which simplifies building Wasm modules. You can install it from *[Wasm](https://rustwasm.github.io/wasm-pack/installer/)*.

##
## Alway run on web server enviorment 

To serve your web application and Wasm module, you can use Python or PHP's built-in web server. Here are the commands:

```
python -m http.server
```

or

```
php -S localhost:8000
```
### use script module 

```
  <script type="module"></script>
```

```
  import { default as wasm, gen_pdf } from "./pkg/pdf.js"; // Update the path to your Wasm module
```


```
function generatePDF(bytes) {
    // Create a blob from the byte array
    const blob = new Blob([bytes], { type: 'application/pdf' });
    // Create a URL for the blob
    const url = URL.createObjectURL(blob);
    return url;
}
```

```
wasm().then(async (module) =>  {
   let pdfByte = await gen_pdf(json_here);
});
```

# example
``
{
    "font_family": "Inter",
    "url":"http://localhost:8000",
    "path":"/fonts/Inter/",
    "elements" : [
        {
            "element": "Text",
            "value":"Quote",
            "font" : "ExtraBold",
            "font_size": 16,
            "x" : 177.4,
            "y": 50.2
        }
    ]
}
``

