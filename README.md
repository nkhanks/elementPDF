## Building Wasm Module:

`wasm-pack build --release --target web`

make sure you install

Ensure Rust is installed. You can download and install it from *[Rust](https://www.rust-lang.org/)*.

Install Wasm-pack, which simplifies building Wasm modules. You can install it from *[Wasm](https://rustwasm.github.io/wasm-pack/installer/)*.

##
## Alway run on web server enviorment 

To serve your web application and Wasm module, you can use Python or PHP's built-in web server. Here are the commands:

```
# Python
python -m http.server

# PHP
php -S localhost:8000

```

### Using Script Module in HTML

In your HTML file, include a script module to import the Wasm module:

```
<script type="module">
    import { default as wasm, gen_pdf } from "./pkg/pdf.js"; // Update the path to your Wasm module
</script>
```


### JavaScript Function to Generate PDF

Define a JavaScript function to generate PDF from the Wasm module's output:

```
function generatePDF(bytes) {
    // Create a blob from the byte array
    const blob = new Blob([bytes], { type: 'application/pdf' });
    // Create a URL for the blob
    const url = URL.createObjectURL(blob);
    return url;
}
```

### Using Wasm Module in JavaScript

Load the Wasm module and call the `gen_pdf` function to generate PDF based on the provided JSON:

```
wasm().then(async (module) =>  {
   let pdfByte = await gen_pdf(json_here);
});
```

### Example JSON Configuration

Here's an example JSON configuration for generating a PDF:

```
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
```
You can replace `json_here` in the JavaScript code with this JSON configuration.

Make sure to adapt the paths and configurations according to your project structure and requirements.

#### Fonts file layout

--/fonts/

------/Inter/

------------/Inter-ExtraBold.tff

------------/Inter-Regular.tff

------------/Inter-Bold.tff

*[Here is font library](https://fonts.google.com/)*.



