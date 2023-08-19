let canvas = document.getElementById("my_canvas");
let gl = canvas.getContext("webgl");
let vertexShader = gl.createShader(gl.VERTEX_SHADER);
gl.shaderSource(vertexShader, `
    attribute vec2 vertex_position;
    void main(void) {
        gl_Position = vec4(vertex_position, 0.0, 1.0);
    }
`);
gl.compileShader(vertexShader);


let fragmentShader = gl.createShader(gl.FRAGMENT_SHADER);
gl.shaderSource(fragmentShader, `
    void main() {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    }
`);
gl.compileShader(fragmentShader);

let shaderProgram = gl.createProgram();
gl.attachShader(shaderProgram, vertexShader);
gl.attachShader(shaderProgram, fragmentShader);
gl.linkProgram(shaderProgram);

let position_attribute_location = gl.getAttribLocation(shaderProgram, "vertex_position");
gl.enableVertexAttribArray(position_attribute_location);
let imports = {
    env: {
        js_clear_screen_to_color: function (red, green, blue, alpha) {
            gl.clearColor(red, green, blue, alpha);
            gl.clear(gl.COLOR_BUFFER_BIT);
        },
        js_clear_screen_to_random_color: function () {
            gl.clearColor(Math.random(), Math.random(), Math.random(), 1.0);
            gl.clear(gl.COLOR_BUFFER_BIT);
        },
        js_draw_rectangle: function (x, y, width, height) {
            let data_buffer = gl.createBuffer();
            gl.bindBuffer(gl.ARRAY_BUFFER, data_buffer);
    
            function position(size, pos) {
                return (pos / size) * 2.0 - 1.0;
            }
            gl.bufferData(
                gl.ARRAY_BUFFER,
                new Float32Array([
                    position(gl.canvas.width, x), position(gl.canvas.height, y),
                    position(gl.canvas.width, x + width), position(gl.canvas.height, y),
                    position(gl.canvas.width, x + width), position(gl.canvas.height, y + height),
                    position(gl.canvas.width, x), position(gl.canvas.height, y + height)
                ]),
                gl.STATIC_DRAW);
    
            gl.vertexAttribPointer(position_attribute_location, 2, gl.FLOAT, false, 0, 0);
    
            gl.useProgram(shaderProgram);
            gl.drawArrays(gl.TRIANGLE_FAN, 0, 4);
            gl.deleteBuffer(data_buffer);
        }
    }
};

WebAssembly.instantiateStreaming(fetch('target/wasm32-unknown-unknown/debug/examples/game.wasm'), imports).then(function (result) {
    result.instance.exports.main();
    document.onkeydown = function (event) {
        let code = 0;
        switch (event.code) {
            case "ArrowLeft":
                code = 1;
                break;
            case "ArrowRight":
                code = 2;
                break;
            case "ArrowUp":
                code = 3;
                break;
            case "ArrowDown":
                code = 4;
                break;
            case "Space":
                code = 5;
                break;
        }
    
        result.instance.exports.key_press(code);
    };
});



