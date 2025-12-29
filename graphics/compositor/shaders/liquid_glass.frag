#version 450

layout(location = 0) in vec2 v_TexCoord;
layout(location = 1) in vec3 v_WorldPos;
layout(location = 2) in vec3 v_Normal;

layout(location = 0) out vec4 o_Color;

// Uniforms
layout(binding = 0) uniform GlobalUniforms {
    float u_Time;         // System time in normalized seconds (0.0 to 86400.0)
    vec2 u_Resolution;    // Screen resolution
    vec3 u_SunColor;      // Calculated by CPU or computed here? Let's compute height here.
};

layout(binding = 1) uniform WindowUniforms {
    float u_Thickness;     // Glass thickness (physical units)
    float u_RefractiveIdx; // Index (e.g. 1.52)
    float u_roughness;     // Surface micro-grain
    float u_Elevation;     // Z-Axis height (0.0 - 10.0)
};

layout(binding = 2) uniform sampler2D u_BackgroundTexture;

const float PI = 3.14159265;

// Gaussian Blur weights
const float weights[5] = float[](0.227027, 0.1945946, 0.1216216, 0.054054, 0.016216);

// ---------------------------------------------------------
// Helper: Calculate Blur based on Z-Elevation
// The higher the window, the blurrier the background.
// ---------------------------------------------------------
vec3 getBlurredBackground(vec2 uv, float elevation) {
    if (elevation <= 0.01) return texture(u_BackgroundTexture, uv).rgb;

    vec2 tex_offset = 1.0 / u_Resolution; // gets size of single texel
    vec3 result = texture(u_BackgroundTexture, uv).rgb * weights[0]; // current fragment's contribution
    
    // Adjust blur radius by elevation
    float radius = elevation * 2.0; 

    for(int i = 1; i < 5; ++i) {
        vec2 offset = vec2(tex_offset.x * i * radius, tex_offset.y * i * radius);
        result += texture(u_BackgroundTexture, uv + vec2(offset.x, 0.0)).rgb * weights[i];
        result += texture(u_BackgroundTexture, uv - vec2(offset.x, 0.0)).rgb * weights[i];
        result += texture(u_BackgroundTexture, uv + vec2(0.0, offset.y)).rgb * weights[i];
        result += texture(u_BackgroundTexture, uv - vec2(0.0, offset.y)).rgb * weights[i];
    }
    return result;
}

// ---------------------------------------------------------
// Helper: Refraction (Snell's Law Approx)
// Thick glass shifts the background image more.
// ---------------------------------------------------------
vec2 calculateRefraction(vec2 uv, vec3 normal, float thickness, float ior) {
    // Simple 2D approximation for UI: 
    // Shift UVs based on XY components of the normal vector
    vec2 offset = normal.xy * thickness * (1.0 - 1.0/ior);
    return uv - offset; // Invert offset for correct visual parallax
}

// ---------------------------------------------------------
// Helper: Chronometric Light Integration
// Calculates tinting based on time of day.
// ---------------------------------------------------------
vec3 applyChronometricLighting(vec3 baseColor, float time) {
    // Normalize time to 24h cycle
    float hour = mod(time / 3600.0, 24.0);
    
    vec3 lightTint = vec3(1.0);
    float ambientIntensity = 1.0;

    if (hour >= 6.0 && hour < 18.0) {
        // Day: Blue/White tint, bright
        float noonDist = abs(hour - 12.0) / 6.0; // 0 at noon, 1 at 6/18
        lightTint = mix(vec3(0.95, 0.98, 1.0), vec3(1.0, 0.8, 0.6), noonDist);
    } else {
        // Night: Cool Blue/Purple tint, dimmer
        lightTint = vec3(0.2, 0.2, 0.4); 
        ambientIntensity = 0.6;
    }

    return baseColor * lightTint * ambientIntensity;
}

// ---------------------------------------------------------
// Main
// ---------------------------------------------------------
void main() {
    // 1. Calculate Refraction UVs
    vec2 refractedUV = calculateRefraction(v_TexCoord, normalize(v_Normal), u_Thickness, u_RefractiveIdx);
    
    // 2. Sample Blurred Background with new UVs
    vec3 background = getBlurredBackground(refractedUV, u_Elevation);
    
    // 3. Apply Glass Tint/Material
    // Glass has a slight inherent color (usually minimal for UI)
    vec3 glassColor = vec3(0.9, 0.95, 1.0); // Very faint blue-white
    vec3 combined = mix(background, glassColor, 0.1); // 10% opacity for the glass itself

    // 4. Apply Specular Highlight (The Sun)
    // We approximate the sun vector based on time
    float hour = mod(u_Time / 3600.0, 24.0);
    float sunAngle = (hour - 6.0) / 12.0 * PI; // 0 to PI
    vec3 sunDir = normalize(vec3(cos(sunAngle), sin(sunAngle), 0.5));
    
    vec3 viewDir = vec3(0.0, 0.0, 1.0); // Orthographic UI usually
    vec3 halfDir = normalize(sunDir + viewDir);
    float spec = pow(max(dot(v_Normal, halfDir), 0.0), 32.0); // Shininess
    
    // Chromatic Aberration on edges (Micro-prism)
    float edgeFactor = length(v_TexCoord - 0.5) * 2.0; // 0 at center, 1 at corners
    if (edgeFactor > 0.8) {
        // Simple RGB split
        float split = 0.005 * edgeFactor;
        combined.r = texture(u_BackgroundTexture, refractedUV + vec2(split, 0.0)).r;
        combined.b = texture(u_BackgroundTexture, refractedUV - vec2(split, 0.0)).b;
    }

    // 5. Final Composition with Chronometric Light
    vec3 finalColor = applyChronometricLighting(combined, u_Time);
    
    // Add specular (Sun reflection is additive, white/gold)
    vec3 sunColor = (hour > 8.0 && hour < 16.0) ? vec3(1.0) : vec3(1.0, 0.6, 0.2);
    finalColor += sunColor * spec * 0.5;

    o_Color = vec4(finalColor, 1.0);
}
