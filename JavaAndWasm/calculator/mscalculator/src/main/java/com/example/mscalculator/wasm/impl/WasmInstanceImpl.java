package com.example.mscalculator.wasm.impl;

import java.io.IOException;
import org.springframework.core.io.Resource;
import org.springframework.core.io.ResourceLoader;
import org.wasmer.Instance;
import com.example.mscalculator.wasm.WasmInstance;

public class WasmInstanceImpl implements WasmInstance {

    private final Instance  instance;

    public WasmInstanceImpl(ResourceLoader resourceLoader, String location) throws IOException {
        Resource resource = resourceLoader.getResource(location);
        byte[] wasmBytes = resource.getContentAsByteArray();
        this.instance = new Instance(wasmBytes);
    }

    @Override
    public Integer sumar(Integer a, Integer b) {
        return (Integer) this.instance.exports
                .getFunction("sumar")
                .apply(a, b)[0];
    }

    @Override
    public Integer restar(Integer a, Integer b) {
        return (Integer) this.instance.exports
                .getFunction("restar")
                .apply(a, b)[0];
    }

    @Override
    public Integer multiplicar(Integer a, Integer b) {
        return (Integer) this.instance.exports
                .getFunction("multiplicar")
                .apply(a, b)[0];
    }

}
