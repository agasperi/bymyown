package com.example.mscalculator.resource;

import com.example.mscalculator.wasm.WasmInstance;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/calculator")
@RequiredArgsConstructor
public class CalculatorResource {

    private final WasmInstance wasmInstance;

    @GetMapping("/sumar")
    public int sumar(@RequestParam int a, @RequestParam int b) {
        return wasmInstance.sumar(a, b);
    }

    @GetMapping("/restar")
    public int restar(@RequestParam int a, @RequestParam int b) {
        return wasmInstance.restar(a, b);
    }

    @GetMapping("/multiplicar")
    public int multiplicar(@RequestParam int a, @RequestParam int b) {
        return wasmInstance.multiplicar(a, b);
    }

}
