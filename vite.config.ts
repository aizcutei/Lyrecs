import { defineConfig } from 'vite'
import Unocss from 'unocss/vite'
import react from '@vitejs/plugin-react'
import reactRefresh from '@vitejs/plugin-react-refresh'
import vitePluginImp from 'vite-plugin-imp'

// https://vitejs.dev/config/
export default defineConfig({
    clearScreen: false,
    server: {
        port: 3000,
        strictPort: true,
    },
    envPrefix: ['VITE_', 'TAURI_'],
    build: {
        // Tauri supports es2021
        target: ['es2021', 'chrome97', 'safari13'],
        // don't minify for debug builds
        minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
        // produce sourcemaps for debug builds
        sourcemap: !!process.env.TAURI_DEBUG,
    },

    plugins: [
        react(),
        reactRefresh(),
        Unocss({
            rules: [
                ['text-stroke-black', { '-webkit-text-stroke-color': 'black' }],
                ['content-text', {'content': `attr(data-text)`}],
                [/^content-text-(\S+)$/, ([ ,d]) => ({ 'content' : `${d}`})],
                [/^text-outline-(\d+)-(\d+)-(\S+)$/, ([, d, w, S]) => ({ 'text-shadow' :
                    `${d as unknown as number * 1}px 0 ${w}px ${S},
                    ${d as unknown as number * 0.9239}px ${d as unknown as number * 0.3827}px ${w}px ${S},
                    ${d as unknown as number * 0.7071}px ${d as unknown as number * 0.7071}px ${w}px ${S},
                    ${d as unknown as number * 0.3827}px ${d as unknown as number * 0.9239}px ${w}px ${S},
                    0 ${d as unknown as number * 1}px ${w}px ${S},
                    -${d as unknown as number * 0.9239}px ${d as unknown as number * 0.3827}px ${w}px ${S},
                    -${d as unknown as number * 0.7071}px ${d as unknown as number * 0.7071}px ${w}px ${S},
                    -${d as unknown as number * 0.3827}px ${d as unknown as number * 0.9239}px ${w}px ${S},
                    -${d as unknown as number * 1}px 0 ${w}px ${S},
                    ${d as unknown as number * 0.9239}px -${d as unknown as number * 0.3827}px ${w}px ${S},
                    ${d as unknown as number * 0.7071}px -${d as unknown as number * 0.7071}px ${w}px ${S},
                    ${d as unknown as number * 0.3827}px -${d as unknown as number * 0.9239}px ${w}px ${S},
                    0 -${d as unknown as number * 1}px ${w}px ${S},
                    -${d as unknown as number * 0.9239}px -${d as unknown as number * 0.3827}px ${w}px ${S},
                    -${d as unknown as number * 0.7071}px -${d as unknown as number * 0.7071}px ${w}px ${S},
                    -${d as unknown as number * 0.3827}px -${d as unknown as number * 0.9239}px ${w}px ${S}`
                })]
                ]
        }),
        vitePluginImp({
            libList: [
                {
                    libName: "antd",
                    style: (name) => `antd/lib/${name}/style/index.less`,
                },
            ],
        }),
    ],
    css: {
        preprocessorOptions: {
            less: {
              // 支持内联 JavaScript
                javascriptEnabled: true,
            }
        }
    },
})
