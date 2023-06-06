package com.example.mscalculator.config;

import com.example.mscalculator.wasm.WasmInstance;
import com.example.mscalculator.wasm.impl.WasmInstanceImpl;
import java.io.IOException;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.core.io.ResourceLoader;

@Configuration
public class WasmConfig {

    @Bean
    WasmInstance wasmInstace (ResourceLoader resourceLoader) throws IOException {
        return new WasmInstanceImpl(resourceLoader, "classpath:calc.wasm");
    }

}
