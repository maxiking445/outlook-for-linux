import { defineConfig } from "vite";
import { resolve } from "path";
import fs from "fs";

export default defineConfig({
  build: {
    outDir: "dist",
  },
  plugins: [
    {
      name: "copy-hooks",
      closeBundle() {
        const srcDir = resolve(__dirname, "src/");
        const distDir = resolve(__dirname, "src-tauri/resources");

        fs.mkdirSync(distDir, { recursive: true });

        fs.readdirSync(srcDir).forEach(file => {
          fs.copyFileSync(
            resolve(srcDir, file),
            resolve(distDir, file)
          );
        });

        console.log("JS files copied to dist/");
      },
    },
  ],
});
