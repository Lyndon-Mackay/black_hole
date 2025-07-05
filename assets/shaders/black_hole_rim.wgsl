#import bevy_pbr::forward_io::VertexOutput



@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let uv = mesh.uv;

    let centered = uv * 2.0 - vec2<f32>(1.0, 1.0);
    let dist = length(centered);




    if dist < 0.5 {
        return vec4<f32>( 1.0,  0.3, 0.0, 1.0);
    }
 
        return vec4<f32>( dist,  0.0, 0.0, 1.0);
}
