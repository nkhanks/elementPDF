## How to build wasm

`wasm-pack build --release --target web`

make sure you install
*[Rust](https://www.rust-lang.org/)*.
*[Wasm](https://rustwasm.github.io/wasm-pack/installer/)*.

##

`
<!-- index.html -->
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Wasm Example</title>
    <script type="module">
         import { default as wasm, gen_pdf } from "./pkg/pdf.js"; // Update the path to your Wasm module

         function generatePDF(bytes) {
    // Create a blob from the byte array
    const blob = new Blob([bytes], { type: 'application/pdf' });

    // Create a URL for the blob
    const url = URL.createObjectURL(blob);

    return url;
}

         wasm().then(async (module) =>  {

            let json = `{
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
`
            let pdfByte = await gen_pdf(json);
        });
    </script>
</head>
<body>
</body>
</html>


`

