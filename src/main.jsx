import "vite/modulepreload-polyfill";

import React, { Suspense } from "react";
import ReactDOM from "react-dom/client";

const root = document.getElementById("root");
const modules = import.meta.glob(`./pages/**/*.jsx`);
const pageData = JSON.parse(root.dataset.page);

function App({ Component, ...props }) {
  return (
    <Suspense fallback={null}>
      <Component {...props} />
    </Suspense>
  )
}

const Component = React.lazy(
  modules[`./pages/${pageData.component}.jsx`] ||
    modules[`./pages/${pageData.component}/index.jsx`]
);

ReactDOM.createRoot(root).render(
  <React.StrictMode>
    <App Component={Component} {...pageData.props} />
  </React.StrictMode>
);
