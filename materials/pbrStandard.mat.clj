{
    :name "pbrStandard"
    :order "Opaque"
    :props [
        {:name "baseColor"            :type "Texture" :default "white" }
        {:name "emissive"             :type "Texture" :default "white" }
        {:name "normal"               :type "Texture" :default "blue" }
        {:name "metallicRoughness"    :type "Texture" :default "white" }
    ]
    :pass [
       
        { 
            :shader { 
                :features ["NormalMap"]
                :name "core.pbr"
                :slot "
                    void slot_fs_material(inout MaterialInputs inputs,vec2 uv,out vec4 normalColor) {
                        inputs.baseColor = texture(sampler2D(tex_baseColor, tex_baseColorSampler), uv);
                        vec4 mr = texture(sampler2D(tex_metallicRoughness, tex_metallicRoughnessSampler), uv);
                        inputs.metallic  = 0.1;
                        inputs.roughness = mr.r;
                        normalColor = texture(sampler2D(tex_normal, tex_normalSampler), uv);
                    }
                 "   
            }
        }

       
        
    ]
}