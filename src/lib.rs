
include!(concat!(env!("OUT_DIR"), "/config.rs"));

ezcfg::pub_cfg! {
    Config [crate::CONFIG_PATH]
        max_dendrite_branches: usize,
        max_branch_receptors: usize,

        min_branch_strength: f32,
        def_branch_strength: f32,
        max_branch_strength: f32,

        min_potential: f32,
        def_potential: f32,

        min_threshold: f32,
        def_threshold: f32,
        max_threshold: f32,

        min_plasticity: f32,
        def_plasticity: f32,
        max_plasticity: f32,

        max_dur_leak: f32,
        min_dur_leak: f32,

        max_dur_hebb: f32,
        min_dur_hebb: f32,

        sensitize_rate: f32,
        plasticize_rate: f32,
        strengthen_rate: f32,
        flatten_rate: f32,
        leak_rate: f32,

        min_ampa: f32,
        def_ampa: f32,
        max_ampa: f32,
        base_ampa: f32,

        min_nmda: f32,
        def_nmda: f32,
        max_nmda: f32,
        base_nmda: f32,

        min_gaba_a: f32,
        def_gaba_a: f32,
        max_gaba_a: f32,
        base_gaba_a: f32,

        min_gaba_b: f32,
        def_gaba_b: f32,
        max_gaba_b: f32,
        base_gaba_b: f32,

        min_nico: f32,
        def_nico: f32,
        max_nico: f32,
        base_nico: f32,

        min_musc: f32,
        def_musc: f32,
        max_musc: f32,
        base_musc: f32,

        min_d1: f32,
        def_d1: f32,
        max_d1: f32,
        base_d1: f32,

        min_d2: f32,
        def_d2: f32,
        max_d2: f32,
        base_d2: f32,

        min_s1: f32,
        def_s1: f32,
        max_s1: f32,
        base_s1: f32,

        min_s2: f32,
        def_s2: f32,
        max_s2: f32,
        base_s2: f32,
}


