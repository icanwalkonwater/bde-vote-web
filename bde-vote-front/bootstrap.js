// Import SCSS
import './scss/index.scss'
// Import WASM package
import('./pkg').then(module => {
    module.run_app();
});
