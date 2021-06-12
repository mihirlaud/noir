import init, { start_game } from './js/wasm.js';

function fix_dpi() {
    let dpi = window.devicePixelRatio;//get canvas
    let canvas = document.getElementById('canvas');//get context

    //get CSS height
    //the + prefix casts it to an integer
    //the slice method gets rid of "px"
    
    let style_height = +getComputedStyle(canvas).getPropertyValue("height").slice(0, -2);
    
    //get CSS width
    let style_width = +getComputedStyle(canvas).getPropertyValue("width").slice(0, -2);
    
    //scale the canvas
    canvas.setAttribute('height', style_height * dpi);
    canvas.setAttribute('width', style_width * dpi);
}

async function run() {
    window.onload = function() {
        fix_dpi();
    };

    await init();
    
    var font = new FontFace("square-font", "url(square.ttf)");
    font.load().then(function (loadedFont){
        document.fonts.add(loadedFont)
        start_game();
    }).catch(function(error) {
        console.log('Failed to load font: ' + error)
    })
}

run();