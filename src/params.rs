use crate::config::LlamaConfigJson;
use crate::tensor::Tensor;
use safetensors::SafeTensors;
pub struct LLamaParams<T> {
    // token_id to embedding lookup table
    pub embedding_table: Tensor<T>, // (vocab_size, dim)
    // decoder layer
    pub rms_att_w: Vec<Tensor<T>>, // (hidden_size, ) x layers
    pub wq: Vec<Tensor<T>>,        // (n_heads * head_size, hidden_size) x layers
    pub wk: Vec<Tensor<T>>,        // (n_kv_heads * head_size, hidden_size) x layers
    pub wv: Vec<Tensor<T>>,        // (n_kv_heads * head_size, hidden_size) x layers
    pub wo: Vec<Tensor<T>>,        // (hidden_size, n_heads * head_size) x layers
    // ffn layer
    pub rms_ffn_w: Vec<Tensor<T>>, // (hidden_size, ) x layers
    pub w_up: Vec<Tensor<T>>,      // (intermediate_size, hidden_size) x layers
    pub w_gate: Vec<Tensor<T>>,    // (intermediate_size, hidden_size) x layers
    pub w_down: Vec<Tensor<T>>,    // (hidden_size, intermediate_size) x layers
    // output
    pub rms_out_w: Tensor<T>, // (hidden_size, )
    pub lm_head: Tensor<T>,   // (vocab_size, dim)
}

impl LLamaParams<f32> {
    pub fn from_safetensors(safetensor: &SafeTensors, config: &LlamaConfigJson) -> Self {
        // println!("{:?}", safetensor.names());
        
        // todo!("实现从safetensors文件的模型参数加载");

        let get_tensor = |name: &str| -> Tensor<f32> {
            let tensor = safetensor.tensor(name).unwrap();
            let data = tensor.data();
            let mut values: Vec<f32> = Vec::with_capacity(data.len() / 4);
            // 将字节数据每4个字节转换为一个 f32 值
            for chunk in data.chunks_exact(4) {
                values.push(f32::from_le_bytes(chunk.try_into().unwrap()));
            }
            Tensor::<f32>::new(values, &tensor.shape().to_vec())
        };

        let layers = config.num_hidden_layers;
        /*
            ["model.layers.1.mlp.up_proj.weight", 
            "model.layers.0.self_attn.o_proj.weight", 
            "model.layers.1.input_layernorm.weight", 
            "model.layers.1.self_attn.q_proj.weight", 
            "model.layers.0.self_attn.k_proj.weight", 
            "model.norm.weight", 
            "lm_head.weight", 
            "model.layers.0.self_attn.q_proj.weight", 
            "model.layers.1.post_attention_layernorm.weight", 
            "model.layers.0.self_attn.v_proj.weight", 
            "model.layers.1.mlp.down_proj.weight", 
            "model.layers.0.input_layernorm.weight", 
            "model.layers.1.self_attn.v_proj.weight", 
            "model.layers.0.mlp.gate_proj.weight", 
            "model.layers.0.mlp.up_proj.weight", 
            "model.layers.0.post_attention_layernorm.weight", 
            "model.layers.1.self_attn.o_proj.weight", 
            "model.layers.0.mlp.down_proj.weight", 
            "model.layers.1.mlp.gate_proj.weight", 
            "model.layers.1.self_attn.k_proj.weight"]
         */
        LLamaParams {
            embedding_table: get_tensor(&format!("lm_head.weight")),

            rms_att_w: (0..layers).map(|i| get_tensor(&format!("model.layers.{}.input_layernorm.weight", i))).collect(),

            wq: (0..layers).map(|i| get_tensor(&format!("model.layers.{}.self_attn.q_proj.weight", i))).collect(),

            wk: (0..layers).map(|i| get_tensor(&format!("model.layers.{}.self_attn.k_proj.weight", i))).collect(),

            wv: (0..layers).map(|i| get_tensor(&format!("model.layers.{}.self_attn.v_proj.weight", i))).collect(),

            wo: (0..layers).map(|i| get_tensor(&format!("model.layers.{}.self_attn.o_proj.weight", i))).collect(),

            rms_ffn_w: (0..layers).map(|i| get_tensor(&format!("model.layers.{}.post_attention_layernorm.weight", i))).collect(),

            w_up: (0..layers).map(|i| get_tensor(&format!("model.layers.{}.mlp.up_proj.weight", i))).collect(),

            w_gate: (0..layers).map(|i| get_tensor(&format!("model.layers.{}.mlp.gate_proj.weight", i))).collect(),

            w_down: (0..layers).map(|i| get_tensor(&format!("model.layers.{}.mlp.down_proj.weight", i))).collect(),

            rms_out_w: get_tensor(&format!("model.norm.weight")),

            lm_head: get_tensor(&format!("lm_head.weight")),
        }
    }
    
}
