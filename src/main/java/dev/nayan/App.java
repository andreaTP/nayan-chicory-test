package dev.nayan;

import java.io.File;

import com.dylibso.chicory.log.Logger;
import com.dylibso.chicory.log.SystemLogger;
import com.dylibso.chicory.runtime.ExportFunction;
import com.dylibso.chicory.runtime.Instance;
import com.dylibso.chicory.runtime.Memory;
import com.dylibso.chicory.runtime.Module;
import com.dylibso.chicory.wasm.types.Value;

public class App 
{

    public static void main( String[] args )
    {
         Instance greetInstance = Module.builder(new File("greet.wasm")).build().instantiate();

         ExportFunction alloc = greetInstance.export("alloc");
         ExportFunction dealloc = greetInstance.export("dealloc");
         ExportFunction greet = greetInstance.export("greet");


         Memory memory = greetInstance.memory();
        
         String a = "golou";

         int len = a.getBytes().length;
         int ptr = alloc.apply(Value.i32(len))[0].asInt();
         memory.writeString(ptr, a);
         var result = greet.apply(Value.i32(ptr),Value.i32(len));
         int newptr = result[0].asInt();
         int newlen = memory.read(newptr);
         System.out.println(memory.readString(newptr + 1, newlen));
    }


}
