{
    :name "pbrStandard"
    :order "Opaque"
    :props [
        {:name "baseColor"            :type "Texture" :default "white" }
        {:name "emissive"             :type "Texture" :default "white" }
        {:name "normal"               :type "Texture" :default "blue" }
        {:name "metallicRoughness"    :type "Texture" :default "white" }

        {:name "metallicFactor" :type "float"  :default 0 }
        {:name "roughnessFactor" :type "float" :default 0 }
        {:name "baseColorFactor" :type "float4" :default [1,1,1,1] }
    ]
    :pass [
       
        { 
            :shader { 
                :features ["NormalMap"]
                :name "core.pbr"
                :slot "
                    void slot_fs_material(inout MaterialInputs inputs,vec2 uv,out vec4 normalColor) {
                        inputs.baseColor = texture(sampler2D(tex_baseColor, tex_baseColorSampler), uv) * material.baseColorFactor;
                        vec4 mr = texture(sampler2D(tex_metallicRoughness, tex_metallicRoughnessSampler), uv);
                        inputs.metallic  = mr.r * material.metallicFactor;
                        inputs.roughness = mr.g * material.roughnessFactor;
                        normalColor = texture(sampler2D(tex_normal, tex_normalSampler), uv);
                    }
                 "   
            }
        }

       
        
    ]
}