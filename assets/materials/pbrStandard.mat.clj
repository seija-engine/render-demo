{
    :name "pbrStandard"
    :order "Opaque"
    :props [
        {:name "baseColor"            :type "Texture" :default "white" }
        {:name "emissive"             :type "Texture" :default "black" }
        {:name "normal"               :type "Texture" :default "blue" }
        {:name "metallicRoughness"    :type "Texture" :default "white" }
        {:name "aoTexture"            :type "Texture" :default "white" }

        {:name "metallicFactor" :type "float"  :default 1 }
        {:name "roughnessFactor" :type "float" :default 1 }
        {:name "baseColorFactor" :type "float4" :default [1,1,1,1] }
        {:name "emissiveFactor" :type  "float3" :default [1,1,1] }
        {:name "alphaCutoff" :type "float" :default 0.1}
    ]
    :pass [
       
        { 
           
            :shader { 
                :features ["NormalMap"]
                :name "core.pbr"
                :slot "
                    void slot_fs_material(inout MaterialInputs inputs,vec2 uv,out vec4 normalColor) {
                        inputs.baseColor = texture(sampler2D(tex_baseColor, tex_baseColorSampler), uv); 
                        inputs.baseColor = inputs.baseColor * material.baseColorFactor;
                        if(inputs.baseColor.a <  material.alphaCutoff) {
                            discard;
                        }
                        vec4 mr = texture(sampler2D(tex_metallicRoughness, tex_metallicRoughnessSampler), uv);
                        inputs.metallic  = mr.r * material.metallicFactor;
                        inputs.roughness = mr.g * material.roughnessFactor;

                        vec4 emissiveColor = texture(sampler2D(tex_emissive, tex_emissiveSampler), uv);
                        inputs.emissiveColor = emissiveColor.rgb * material.emissiveFactor;
                        normalColor = texture(sampler2D(tex_normal, tex_normalSampler), uv);

                        inputs.occlusion = texture(sampler2D(tex_aoTexture, tex_aoTextureSampler), uv).r; 
                    }
                 "   
            }
        }

       
        
    ]
}