import {mat4} from 'gl-matrix'

import * as state from './state'
import * as transforms from './transforms'
import {Camera, ProgramInfo, Shape} from './interfaces'

// WebGl reference:
// https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Adding_2D_content_to_a_WebGL_context

function initShaderProgram(gl: any, vsSource: any, fsSource: any) {
    // Initialize a shader program, so WebGL knows how to draw our data

    const vertexShader = loadShader(gl, gl.VERTEX_SHADER, vsSource)
    const fragmentShader = loadShader(gl, gl.FRAGMENT_SHADER, fsSource)

    // Create the shader program
    const shaderProgram = gl.createProgram()
    gl.attachShader(shaderProgram, vertexShader)
    gl.attachShader(shaderProgram, fragmentShader)
    gl.linkProgram(shaderProgram)

    // If creating the shader program failed, alert
    if (!gl.getProgramParameter(shaderProgram, gl.LINK_STATUS)) {
        alert('Unable to initialize the shader program: ' + gl.getProgramInfoLog(shaderProgram))
        return null
    }

    return shaderProgram
}

function loadShader(gl: any, type: any, source: any) {
    // creates a shader of the given type, uploads the source and
    // compiles it.
    const shader = gl.createShader(type)

    // Send the source to the shader object
    gl.shaderSource(shader, source)

    // Compile the shader program
    gl.compileShader(shader)

    // See if it compiled successfully
    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        alert('An error occurred compiling the shaders: ' + gl.getShaderInfoLog(shader))
        gl.deleteShader(shader)
        return null
    }

    return shader
}

function drawScene(
    gl: any,
    programInfo: ProgramInfo,
    staticBuffers: any,
    pfBuffers: any,
    // modelViewMat: Float32Array,
    viewMatrix: Float32Array,
    projectionMatrix: Float32Array,
    shapes: Map<number, Shape>,
) {

    // Clear the canvas before we start drawing on it.
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT)

    // Skybox
    // {
    //     // We'll supply texcoords as floats.
    //     gl.vertexAttribPointer(
    //         programInfo.attribLocations.skyboxTexCoords,
    //         2,
    //         gl.FLOAT,
    //         false,
    //         0,
    //         0
    //     )
    //     gl.enableVertexAttribArray(programInfo.attribLocations.skyboxTexCoords)
    // }

    shapes.forEach(
        (shape, s_id, map_) => {
            // Create a perspective matrix, a special matrix that is
            // used to simulate the distortion of perspective in a camera.
            // Our field of view is 45 degrees, with a width/height
            // ratio that matches the display size of the canvas
            // and we only want to see objects between 0.1 units
            // and 100 units away from the camera.

            // Set the drawing position to the "identity" point, which is
            // the center of the scene.

            // We've positioned our points rel to their model and the cam already,
            // using 4d transforms; doesn't modify further.

            // Tell WebGL how to pull out the positions from the position
            // buffer into the vertexPosition attribute.
            {
                const numComponents = 4  // pull out 4 values per iteration
                const type = gl.FLOAT    // the data in the buffer is 32bit floats
                const normalize = false  // don't normalize
                const stride = 0         // how many bytes to get from one set of values to the next
                                         // 0 = use type and numComponents above
                const offset = 0         // how many bytes inside the buffer to start from

                gl.bindBuffer(gl.ARRAY_BUFFER, pfBuffers.get(s_id).vertexPosition)
                gl.vertexAttribPointer(
                    programInfo.attribLocations.vertexPosition,
                    numComponents,
                    type,
                    normalize,
                    stride,
                    offset)
                gl.enableVertexAttribArray(programInfo.attribLocations.vertexPosition)
            }

            gl.bindBuffer(gl.ARRAY_BUFFER, pfBuffers.get(s_id).shapePosition)
            gl.vertexAttribPointer(
                programInfo.attribLocations.shapePosition,
                4,
                gl.FLOAT ,
                false ,
                0,
                0
            )
            gl.enableVertexAttribArray(programInfo.attribLocations.shapePosition)

            gl.bindBuffer(gl.ARRAY_BUFFER, pfBuffers.get(s_id).camPosition)
            gl.vertexAttribPointer(
                programInfo.attribLocations.camPosition,
                4,
                gl.FLOAT ,
                false ,
                0,
                0
            )
            gl.enableVertexAttribArray(programInfo.attribLocations.camPosition)

            // Tell WebGL how to pull out the colors from the color buffer
            // into the vertexColor attribute.
            gl.bindBuffer(gl.ARRAY_BUFFER, pfBuffers.get(s_id).uDist)
            gl.vertexAttribPointer(
                programInfo.attribLocations.uDist,
                1,  // Rather than 4 elements, we have one color per vertex.
                gl.FLOAT ,
                false ,
                0,
                0
            )
            gl.enableVertexAttribArray(programInfo.attribLocations.uDist)

            // Tell WebGL which indices to use to index the vertices
            gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, staticBuffers.indexBuffers.get(s_id))

            // Tell WebGL to use our program when drawing
            gl.useProgram(programInfo.program)

            // Set the shader uniforms
            gl.uniformMatrix4fv(
                programInfo.uniformLocations.projectionMatrix,
                false,  // transponse is always false for WebGl.
                projectionMatrix
            )

            // gl.uniformMatrix4fv(
            //     programInfo.uniformLocations.modelViewMatrix,
            //     false,
            //     modelViewMat)

            gl.uniformMatrix4fv(
                programInfo.uniformLocations.modelMatrix,
                false,
                transforms.makeModelMat4(shape)
            )
            gl.uniformMatrix4fv(
                programInfo.uniformLocations.viewMatrix,
                false,
                viewMatrix
            )

            {
                const type = gl.UNSIGNED_SHORT
                const offset = 0
                const vertexCount = shape.get_tris().length

                gl.drawElements(gl.TRIANGLES, vertexCount, type, offset)
            }
        }
    )
}

export function makeStaticBuffers(gl: any, shapes_: Map<number, Shape>, skybox_: Shape) {
    // Create a buffer for our shapes' positions and color.
    const sbPositions: any = []
    let sbVertex
    skybox_.faces_vert.map(face => face.map(vertex_i => {
        sbVertex = (skybox_.nodes.get(vertex_i) as any).a
        for (let i=0; i < 4; i++) {
            sbPositions.push(sbVertex[i])

        }
    }))

    // Now pass the list of positions into WebGL to build the
    // shape. We do this by creating a Float32Array from the
    // JavaScript array, then use it to fill the current buffer.
    const skyboxPositBuffer = gl.createBuffer()
    gl.bindBuffer(gl.ARRAY_BUFFER, skyboxPositBuffer)
    gl.bufferData(gl.ARRAY_BUFFER,
        new Float32Array(sbPositions),
        gl.STATIC_DRAW)

    // todo skybox texture wip
    // look up where the vertex data needs to go.
    // const positionLocation = gl.getAttribLocation(shaderProgram, "a_position");
    // consttexcoordLocation = gl.getAttribLocation(program, "a_texcoords");
    // Create a buffer for texcoords.

    const skyboxTexBuffer = gl.createBuffer()
    gl.bindBuffer(gl.ARRAY_BUFFER, skyboxTexBuffer)
    // Set Texcoords.
    gl.bufferData(
        gl.ARRAY_BUFFER,
        new Float32Array([
            // left column front
            0, 0,
            0, 1,
            1, 0,
            0, 1,
            1, 1,
            1, 0,

            // top rung front
            0, 0,
            0, 1,
            1, 0,
            0, 1,
            1, 1,
            1, 0,
        ]),
        gl.STATIC_DRAW)

    // Build the element array buffer; this specifies the indices
    // into the vertex arrays for each face's vertices.

    // Build the element array buffer; this specifies the indices
    // into the vertex arrays for each face's vertices.

    // This array defines each face as two triangles, using the
    // indices into the vertex array to specify each triangle's
    // position.
    let indices: number[] = []
    let indexModifier = 0
    let tri_indices, indexBuffer
    let indexBuffers = new Map()

    shapes_.forEach(
        (shape, s_id, map_) => {
            indexBuffer = gl.createBuffer()
            gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer)

            // This array defines each face as two triangles, using the
            // indices into the vertex array to specify each triangle's
            // position.
            indices = []
            indexModifier = 0
            tri_indices = shape.get_tris().map(ind => ind + indexModifier)
            indices.push(...tri_indices)
            indexModifier += shape.numFaceVerts()

            // Now send the element array to GL
            gl.bufferData(gl.ELEMENT_ARRAY_BUFFER,
                new Uint16Array(indices), gl.STATIC_DRAW)

            indexBuffers.set(s_id, indexBuffer)
        })

    // Now send the element array to GL
    gl.bufferData(gl.ELEMENT_ARRAY_BUFFER,
        new Uint16Array(indices), gl.STATIC_DRAW)

    return {
        indexBuffers: indexBuffers,
        skybox: skyboxTexBuffer,
        skyboxPosits: skyboxPositBuffer
    }
}

function makePerFrameBuffers(gl: any, shapes: Map<number, Shape>, cam: Camera):
    Map<number, any> {
    let vertexPositionBuffer, uDistBuffer, shapePositionBuffer, camPositionBuffer,
        vertexPositions, faceColors, vertex, dists, shapePositDuped, camPositDuped

    let result = new Map()

    shapes.forEach(
        (shape, s_id, map_) => {
            // modelMat = transforms.makeModelMat(shape)
            // dotMM5(modelViewMat, viewMat, modelMat)

            // Now create an array of positions for the shapes.
            vertexPositions = []
            faceColors = []

            // todo we shouldn't need to create these superfluous shape and cam
            // todo posit arrays... Find a better way
            shapePositDuped = []
            camPositDuped = []
            dists = []
            // Set up vertices.
            for (let face of shape.faces_vert) {
                for (let vertex_i of face) {
                    vertex = (shape.nodes.get(vertex_i) as any).a
                    for (let i=0; i < 4; i++) {
                        vertexPositions.push(vertex[i])
                        shapePositDuped.push(shape.position[i])
                        camPositDuped.push(cam.position[i])
                    }
                    dists.push(cam.position[3] - vertex[3])  // u dist.
                }
            }

            vertexPositionBuffer = gl.createBuffer()
            gl.bindBuffer(gl.ARRAY_BUFFER, vertexPositionBuffer)
            gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertexPositions), gl.STATIC_DRAW)

            uDistBuffer = gl.createBuffer()
            gl.bindBuffer(gl.ARRAY_BUFFER, uDistBuffer)
            gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(dists), gl.STATIC_DRAW)

            // quick position is the shape's whole position; eg just 4 elements.
            // used for our separate model, view logic.
            shapePositionBuffer = gl.createBuffer()
            gl.bindBuffer(gl.ARRAY_BUFFER, shapePositionBuffer)
            gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(shapePositDuped), gl.STATIC_DRAW)

            camPositionBuffer = gl.createBuffer()
            gl.bindBuffer(gl.ARRAY_BUFFER, camPositionBuffer)
            gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(camPositDuped), gl.STATIC_DRAW)

            result.set(s_id, {
                vertexPosition: vertexPositionBuffer,
                uDist: uDistBuffer,
                shapePosition: shapePositionBuffer,
                // todo we don't need to add the cam to the buffer each shape;
                // todo need to rethink how we organize our buffer-creation funcs.
                camPosition: camPositionBuffer
            })
        }
    )

    return result
}

export function gl_main() {
    // Initialize WebGL rendering.
    const canvas = document.getElementById("glCanvas")
    const gl = (canvas as any).getContext("webgl")

    // Overall settings here.
    gl.clearColor(0.0, 0.0, 0.0, 1.0)  // Clear to black, fully opaque

    // These settings affect transparency.
    gl.clearDepth(1.0)                 // Clear everything
    // gl.enable(gl.DEPTH_TEST)           // Enable depth testing
    // gl.depthFunc(gl.LEQUAL)            // Near things obscure far things
    gl.disable(gl.DEPTH_TEST);
    // gl.disable(gl.CULL_FACE)  // transparency TS

    gl.enable(gl.BLEND);
    gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA)
    // gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA)
    // gl.blendFuncSeparate(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA, gl.ONE, gl.ONE_MINUS_SRC_ALPHA);

    // gl.blendFunc(gl.SRC_ALPHA, gl.ONE);

    if (!gl) {
        alert("Unable to initialize WebGL. Your browser or machine may not support it.")
    }

    document.onkeyup = transforms.handleKeyUp

    // Vertex shader program
    const vsSource = `
        attribute vec4 aVertexPosition;
        // attribute vec4 aVertexColor;
        attribute float aDist;
        
        attribute vec4 aShapePosition;
        attribute vec4 aCamPosition;
        
        uniform mat4 uProjectionMatrix;
                
        // We can't pass 5x5 homogenous matrices to the shader, but can pass 4x4,
        // non-homogenous matrices, then translate separately.
        uniform mat4 uModelMatrix;
        uniform mat4 uViewMatrix;
               
        varying lowp vec4 vColor;
    
        void main() {
            vec4 positionedPt;
            // For model transform, position after the transform
            positionedPt = (uModelMatrix * aVertexPosition) + aShapePosition;
            // for view transform, position first.
            positionedPt = uViewMatrix * (positionedPt - aCamPosition);
            
            // Now remove the u coord; replace with one. We no longer need it, 
            // and the projection matrix is set up for 3d homogenous vectors.
            vec4 positioned3d = vec4(positionedPt[0], positionedPt[1], positionedPt[2], 1.);
            
            gl_Position = uProjectionMatrix * positioned3d;
          
            // Now calculate the color, based on passed u dist from cam.
            vec4 calced_color;
            
            // todo pass in colormax.
            float portion_through = abs(aDist) / 1.5;

            if (portion_through > 1.) {
                portion_through = 1.;
            }
            
            float baseGray = 0.0;
            float colorVal = baseGray + portion_through * 1. - baseGray;
            
            if (aDist > 0.) {
                calced_color = vec4(baseGray, baseGray, colorVal, 0.2);  // Blue
            } else {
                calced_color = vec4(colorVal, baseGray, baseGray, 0.2);  // Red
            }

            vColor = calced_color;
        }
    `

    // Fragment shader program
    const fsSource = `
        varying lowp vec4 vColor;


        void main() {
            gl_FragColor = vColor;
        }
    `

    // Vertex shader program
    const vsSkybox = `
        attribute vec4 a_position;
        attribute vec2 a_texcoord;
         
        uniform mat4 u_matrix;
         
        varying vec2 v_texcoord;
         
        void main() {
          // Multiply the position by the matrix.
          gl_Position = u_matrix * a_position;
         
          // Pass the texcoord to the fragment shader.
          v_texcoord = a_texcoord;
        }
    `

    // Fragment shader program
    const fsSkybox = `
        precision mediump float;
         
        // Passed in from the vertex shader.
        varying vec2 v_texcoord;
         
        // The texture.
        uniform sampler2D u_texture;
         
        void main() {
           gl_FragColor = texture2D(u_texture, v_texcoord);
        }
    `

    // Initialize a shader program; this is where all the lighting
    // for the vertices and so forth is established.
    const shaderProgram = initShaderProgram(gl, vsSource, fsSource)
    const shaderSkybox = initShaderProgram(gl, vsSkybox, fsSkybox)

    // Collect all the info needed to use the shader program.
    // Look up which attribute our shader program is using
    // for aVertexPosition and look up uniform locations.
    const programInfo = {
        program: shaderProgram,
        attribLocations: {
            vertexPosition: gl.getAttribLocation(shaderProgram, 'aVertexPosition'),
            shapePosition: gl.getAttribLocation(shaderProgram, 'aShapePosition'),
            camPosition: gl.getAttribLocation(shaderProgram, 'aCamPosition'),
            uDist: gl.getAttribLocation(shaderProgram, 'aDist'),
            skyboxTexCoords: gl.getAttribLocation(shaderSkybox, 'a_texcoord'),
        },
        uniformLocations: {
            projectionMatrix: gl.getUniformLocation(shaderProgram, 'uProjectionMatrix'),
            // modelViewMatrix: gl.getUniformLocation(shaderProgram, 'uModelViewMatrix'),
            modelMatrix: gl.getUniformLocation(shaderProgram, 'uModelMatrix'),
            viewMatrix: gl.getUniformLocation(shaderProgram, 'uViewMatrix'),
        },
    }

    let then = 0
    // These buffers don't change; eg index buffers.  todo put static buffers back here?!
    // let staticBuffers = initBuffers(gl, state.shapes, state.skybox)

    // Note: If we want dynamically-adjustable FOV, we need to move this,
    // or part of it to drawScene.
    let projectionMatrix = mat4.create()
    // note: glmatrix.js always has the first argument
    // as the destination to receive the result.
    mat4.perspective(
        projectionMatrix,
        state.cam.fov,
        state.cam.aspect,
        state.cam.near,
        state.cam.far
    )

    // Draw the scene repeatedly
    function render(now: number) {
        now *= 0.001;  // convert to seconds
        const deltaTime = now - then;
        then = now;

        // Here's where we call the routine that builds all the
        // objects we'll be drawing.
        // const processedShapes = transforms.processShapes(state.cam, state.shapes)
        // pfBuffers = perFrameBuffers(gl, processedShapes)

        // const viewMatrix = transforms.makeViewMat(state.cam)
        const viewMatrix = transforms.makeViewMat4(state.cam)

        // This is called when we change the shapes, and on init.
        if (Object.keys(state.staticBuffers).length === 0) {
            state.updateStaticBuffers(gl, makeStaticBuffers(gl, state.shapes, state.skybox))
        }

        const pfBuffers = makePerFrameBuffers(gl, state.shapes, state.cam)

        drawScene(gl, programInfo, state.staticBuffers, pfBuffers,
            viewMatrix, projectionMatrix,
            state.shapes)

        requestAnimationFrame(render)

        // Update the rotation for the next draw
        state.shapes.forEach(
            (shape, id, map) => {
                // todo need vector addition to simplify...
                shape.orientation[0] += shape.rotation_speed[0]
                shape.orientation[1] += shape.rotation_speed[1]
                shape.orientation[2] += shape.rotation_speed[2]
                shape.orientation[3] += shape.rotation_speed[3]
                shape.orientation[4] += shape.rotation_speed[4]
                shape.orientation[5] += shape.rotation_speed[5]
            }
        )
    }
    requestAnimationFrame(render)
}
