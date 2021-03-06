import * as input from './input'
import * as shaders from './shaders'
import * as state from './state'
import {Camera, ProgramInfo, Shape} from './types'

// import * as transforms from './transforms'

// WebGl reference:
// https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Adding_2D_content_to_a_WebGL_context

function initShaderProgram(gl: WebGLRenderingContext, vsSource: string, fsSource: string) {
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

function loadShader(gl: WebGLRenderingContext, type: number, source: string) {
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
    gl: WebGLRenderingContext,
    programInfo: ProgramInfo,
    staticBuffers: any,
    pfBuffers: any,
    // skyboxPositBuffer: any,
    viewMatrix: Float32Array,
    projectionMatrix: Float32Array,
    shapes: Map<number, Shape>,
    modelMatMaker: Function,
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

                gl.bindBuffer(gl.ARRAY_BUFFER, staticBuffers.vertexBuffers.get(s_id))
                gl.vertexAttribPointer(
                    programInfo.attribLocations.vertexPosition,
                    numComponents,
                    type,
                    normalize,
                    stride,
                    offset)
                gl.enableVertexAttribArray(programInfo.attribLocations.vertexPosition)
            }
            gl.bindBuffer(gl.ARRAY_BUFFER, staticBuffers.normalBuffers.get(s_id))
            gl.vertexAttribPointer(
                programInfo.attribLocations.normal,
                4,
                gl.FLOAT,
                false,
                0,
                0
            )
            gl.enableVertexAttribArray(programInfo.attribLocations.normal)

            // Tell WebGL how to pull out the colors from the color buffer
            // into the vertexColor attribute.

            // Tell WebGL which indices to use to index the vertices
            gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, staticBuffers.indexBuffers.get(s_id))

            // Tell WebGL to use our program when drawing
            gl.useProgram(programInfo.program)

            // Set the shader uniforms

            let modelMat  = modelMatMaker(shape.orientation, shape.scale)
            // let modelMatJs = transforms.makeModelMat4(shape.orientation, shape.scale)

            gl.uniformMatrix4fv(
                programInfo.uniformLocations.modelMatrix,
                false,
                // modelMatMaker(shape.orientation, shape.scale)
                // transforms.makeModelMat4(shape)
                modelMat
            )
            gl.uniformMatrix4fv(
                programInfo.uniformLocations.viewMatrix,
                false,
                viewMatrix
            )
            gl.uniformMatrix4fv(
                programInfo.uniformLocations.projectionMatrix,
                false,  // transponse is always false for WebGl.
                projectionMatrix
            )

            gl.uniform4fv(programInfo.uniformLocations.shapePosition,
                new Float32Array(shape.position))
            gl.uniform4fv(programInfo.uniformLocations.camPosition,
                new Float32Array(state.scene.cam.position))
            gl.uniform4fv(programInfo.uniformLocations.ambientColor,
                state.scene.lighting.ambient_color)
            gl.uniform4fv(programInfo.uniformLocations.diffuseColor,
                state.scene.lighting.diffuse_color)
            gl.uniform4fv(programInfo.uniformLocations.diffuseDirection,
                state.scene.lighting.diffuse_direction)

            gl.uniform1f(programInfo.uniformLocations.ambientIntensity,
                state.scene.lighting.ambient_intensity)
            gl.uniform1f(programInfo.uniformLocations.diffuseIntensity,
                state.scene.lighting.diffuse_intensity)
            gl.uniform1f(programInfo.uniformLocations.colorMax, state.scene.color_max)
            gl.uniform1f(programInfo.uniformLocations.shapeOpacity, shape.opacity)

            {
                const type = gl.UNSIGNED_SHORT
                const offset = 0
                const vertexCount = shape.mesh.tris.length

                gl.drawElements(gl.TRIANGLES, vertexCount, type, offset)
            }
        }
    )
}

export function makeStaticBuffers(gl: WebGLRenderingContext, shapes_: Map<number, Shape>) {
    // Create a buffer for our shapes' positions and color.
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
    let indices: number[] = [], normals, vertices, face, vertex
    let indexModifier = 0
    let tri_indices, indexBuffer, vertexBuffer, normalBuffer
    let indexBuffers = new Map()
    let normalBuffers = new Map()
    let vertexBuffers = new Map()
    // todo do we need to update normals each frame, or is once per shape suffient?
    shapes_.forEach(
        (shape, s_id, map_) => {

            // This array defines each face as two triangles, using the
            // indices into the vertex array to specify each triangle's
            // position.
            indices = []
            indexModifier = 0
            tri_indices = shape.mesh.tris.map(ind => ind + indexModifier)
            indices.push(...tri_indices)
            indexModifier += shape.mesh.numFaceVerts()

            // Now send the element array to GL.  ELEMENT_ARRAY_BUFFER is used for indices.
            indexBuffer = gl.createBuffer()
            gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer)
            gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, new Uint16Array(indices), gl.STATIC_DRAW)
            indexBuffers.set(s_id, indexBuffer)

            vertices = []
            normals = []
            for (let i=0; i < shape.mesh.faces_vert.length; i++) {
                face = shape.mesh.faces_vert[i]
                for (let vertId of face) {
                    vertex = (shape.mesh.vertices.get(vertId) as any).position
                    for (let coord = 0; coord < 4; coord++) {  // Iterate through each coord.
                        vertices.push(vertex[coord])
                        normals.push(shape.mesh.normals[i].normal[coord])
                    }
                }
            }

            vertexBuffer = gl.createBuffer()
            // ARRAY_BUFFER is used for indexed content; we don't need to take
            // triangles into account.
            gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuffer)
            gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW)
            vertexBuffers.set(s_id, vertexBuffer)

            normalBuffer = gl.createBuffer()
            gl.bindBuffer(gl.ARRAY_BUFFER, normalBuffer)
            gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(normals), gl.STATIC_DRAW)
            normalBuffers.set(s_id, normalBuffer)
        }
    )

    // Now send the element array to GL
    gl.bufferData(gl.ELEMENT_ARRAY_BUFFER,
        new Uint16Array(indices), gl.STATIC_DRAW)

    return {
        indexBuffers: indexBuffers,
        vertexBuffers: vertexBuffers,
        normalBuffers: normalBuffers,
        skybox: skyboxTexBuffer,
    }
}

function makePerFrameBuffers(gl: WebGLRenderingContext, shapes: Map<number, Shape>, cam: Camera):
    Map<number, any> {
    let shapePositionBuffer, camPositionBuffer,
        faceColors, vertex, dists, shapePositDuped, camPositDuped

    let result = new Map()

    shapes.forEach(
        (shape, s_id, map_) => {
            // Now create an array of positions for the shapes.
            faceColors = []

            // todo we shouldn't need to create these superfluous shape and cam
            // todo posit arrays... Find a better way.  Eg uniforms.
            shapePositDuped = []
            camPositDuped = []
            // Set up vertices.
            for (let face of shape.mesh.faces_vert) {
                for (let vertex_i of face) {
                    for (let i=0; i < 4; i++) {
                        shapePositDuped.push(shape.position[i])
                        camPositDuped.push(cam.position[i])
                    }
                }
            }

            // quick position is the shape's whole position; eg just 4 elements.
            // used for our separate model, view logic.
            shapePositionBuffer = gl.createBuffer()
            gl.bindBuffer(gl.ARRAY_BUFFER, shapePositionBuffer)
            gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(shapePositDuped), gl.STATIC_DRAW)

            camPositionBuffer = gl.createBuffer()
            gl.bindBuffer(gl.ARRAY_BUFFER, camPositionBuffer)
            gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(camPositDuped), gl.STATIC_DRAW)

            result.set(s_id, {
                shapePosition: shapePositionBuffer,
                // todo we don't need to add the cam to the buffer each shape;
                // todo need to rethink how we organize our buffer-creation funcs.
                camPosition: camPositionBuffer
            })
        }
    )
    return result
}

// function makeSkyboxPositBuffer(gl: WebGLRenderingContext, skybox: Shape, cam: Camera) {
//     // Run this every frame.
//      // Now process the skybox positions.
//
//     const positions: any = []
//     let vertex
//     skybox.mesh.faces_vert.map(face => face.map(vertex_i => {
//         vertex = (skybox.nodes.get(vertex_i) as any).a
//         for (let i=0; i < 4; i++) {
//             positions.push(vertex[i])
//         }
//     }))
//
//     // Now pass the list of positions into WebGL to build the
//     // shape. We do this by creating a Float32Array from the
//     // JavaScript array, then use it to fill the current buffer.
//     const buffer = gl.createBuffer()
//     gl.bindBuffer(gl.ARRAY_BUFFER, buffer)
//     gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW)
//
//     return buffer
// }

export function main(viewMatMaker: Function, modelMatMaker: Function,
                     makeRotator: Function, makeProj: Function) {
    // Initialize WebGL rendering.
    const canvas = document.getElementById("glCanvas")
    const gl = (canvas as any).getContext("webgl")

    gl.clearColor(0.0, 0.0, 0.0, 1.0)  // Clear to black, fully opaque
    gl.clearDepth(1.0)                 // Clear everything

    gl.enable(gl.BLEND);
    gl.disable(gl.CULL_FACE)
    gl.disable(gl.DEPTH_TEST)
    gl.disable(gl.DITHER)
    gl.disable(gl.POLYGON_OFFSET_FILL)
    gl.disable(gl.SAMPLE_ALPHA_TO_COVERAGE)
    gl.disable(gl.SAMPLE_COVERAGE)
    gl.disable(gl.SCISSOR_TEST)
    gl.disable(gl.STENCIL_TEST)

    gl.depthFunc(gl.LEQUAL)            // Near things obscure far things
    gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);

    if (!gl) {
        alert("Unable to initialize WebGL. Your browser or machine may not support it.")
    }

    document.onkeyup = e => input.handleKeyUp(e)
    document.onkeydown = e => input.handleKeyDown(e)

    // Initialize a shader program; this is where all the lighting
    // for the vertices and so forth is established.
    const shaderProgram = initShaderProgram(gl, shaders.vsSource, shaders.fsSource)
    const shaderSkybox = initShaderProgram(gl, shaders.vsSkybox, shaders.fsSkybox)

    // Collect all the info needed to use the shader program.
    // Look up which attribute our shader program is using
    // for a_vertex_position and look up uniform locations.
    const programInfo = {
        program: shaderProgram,
        skyboxProgram: shaderSkybox,
        attribLocations: {
            vertexPosition: gl.getAttribLocation(shaderProgram, 'position'),
            normal: gl.getAttribLocation(shaderProgram, 'normal'),
            // skyboxTexCoords: gl.getAttribLocation(shaderSkybox, 'a_texcoord'),
        },
        uniformLocations: {
            projectionMatrix: gl.getUniformLocation(shaderProgram, 'u_proj'),
            modelMatrix: gl.getUniformLocation(shaderProgram, 'u_model'),
            viewMatrix: gl.getUniformLocation(shaderProgram, 'u_view'),

            shapePosition: gl.getUniformLocation(shaderProgram, 'u_shape_position'),
            camPosition: gl.getUniformLocation(shaderProgram, 'u_cam_position'),
            ambientColor: gl.getUniformLocation(shaderProgram, 'u_ambient_color'),
            diffuseColor: gl.getUniformLocation(shaderProgram, 'u_diffuse_color'),
            diffuseDirection: gl.getUniformLocation(shaderProgram, 'u_diffuse_direction'),

            ambientIntensity: gl.getUniformLocation(shaderProgram, 'u_ambient_intensity'),
            diffuseIntensity: gl.getUniformLocation(shaderProgram, 'u_diffuse_intensity'),
            colorMax: gl.getUniformLocation(shaderProgram, 'u_color_max'),
            shapeOpacity: gl.getUniformLocation(shaderProgram, 'u_shape_opacity'),
        },
    }

    let then = 0
    // These buffers don't change; eg index buffers.  todo put static buffers back here?!
    // let staticBuffers = initBuffers(gl, state.shapes, state.skybox)

    // Note: If we want dynamically-adjustable FOV, we need to move this,
    // or part of it to drawScene.
    // let projectionMatrix = mat4.create()
    // // note: glmatrix.js always has the first argument
    // // as the destination to receive the result.
    // mat4.perspective(
    //     projectionMatrix,
    //     state.scene.cam.fov,
    //     state.scene.cam.aspect,
    //     state.scene.cam.near,
    //     state.scene.cam.far
    // )

    // let projectionMatrix = transforms.makeProjMat(state.scene.cam)
    let projectionMatrix = makeProj(state.scene.cam)

    // modelMatMaker(state.scene.shapes.get(0).orientation, state.scene.shapes.get(0).scale)

    // Draw the scene repeatedly
    function render(now: number) {
        now *= 0.001;  // convert to seconds
        const deltaTime = now - then;
        then = now;

        input.handlePressed(makeRotator, state.currentlyPressedKeys, deltaTime,
                            state.scene.sensitivities[0], state.scene.sensitivities[1],
                            state.scene.cam_type)

        // Here's where we call the routine that builds all the
        // objects we'll be drawing.

        const viewMatrix = viewMatMaker(state.scene.cam.θ)
        // const viewMatrixJ = transforms.makeViewMat4(state.scene.cam.θ)

        // This is called when we change the shapes, and on init.
        if (Object.keys(state.staticBuffers).length === 0) {
            state.updateStaticBuffers(gl, makeStaticBuffers(gl, state.scene.shapes))
        }

        // const pfBuffers = makePerFrameBuffers(gl, state.shapes, state.cam)
        const pfBuffers = {}
        // const skyboxPositBuffer = makeSkyboxPositBuffer(gl, state.skybox, state.cam)

        drawScene(gl, programInfo, state.staticBuffers, pfBuffers,
            viewMatrix, projectionMatrix,
            state.scene.shapes, modelMatMaker)

        // viewMatrix.free()

        requestAnimationFrame(render)

        // Update the rotation for the next draw
        state.scene.shapes.forEach(
            (shape, id, map) => {
                // todo need vector addition to simplify...
                shape.orientation[0] += shape.rotation_speed[0] * deltaTime
                shape.orientation[1] += shape.rotation_speed[1] * deltaTime
                shape.orientation[1] += shape.rotation_speed[1] * deltaTime
                shape.orientation[2] += shape.rotation_speed[2] * deltaTime
                shape.orientation[3] += shape.rotation_speed[3] * deltaTime
                shape.orientation[4] += shape.rotation_speed[4] * deltaTime
                shape.orientation[5] += shape.rotation_speed[5] * deltaTime
            }
        )
    }
    requestAnimationFrame(render)
}
