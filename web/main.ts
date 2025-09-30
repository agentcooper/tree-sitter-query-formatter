import { format } from "./transpiled/tree_sitter_query_formatter.js";
import * as monaco from "monaco-editor";
import {
  compressToEncodedURIComponent,
  decompressFromEncodedURIComponent,
} from "lz-string";

let inputEditor: monaco.editor.IStandaloneCodeEditor;
let outputEditor: monaco.editor.IStandaloneCodeEditor;

const INITIAL_VALUE = `[(class_declaration name: (identifier) @the-class-name body: (class_body (method_definition name: (property_identifier) @the-method-name)))
(comment (_))]`;

const editorConfig: monaco.editor.IStandaloneEditorConstructionOptions = {
  value: "",
  language: "scheme",
  theme: "vs",
  automaticLayout: true,
  minimap: { enabled: false },
  scrollBeyondLastLine: false,
  wordWrap: "on",
  fontSize: 14,
  fontFamily: "monospace",
};

function setQueryInUrl(query: string) {
  if (query) {
    const compressed = compressToEncodedURIComponent(query);
    window.location.hash = compressed;
  } else {
    window.location.hash = "";
  }
}

function getQueryFromUrl(): string | null {
  const hash = window.location.hash.slice(1);
  if (!hash) return null;

  try {
    return decompressFromEncodedURIComponent(hash);
  } catch (error) {
    console.warn("Failed to decompress URL hash:", error);
    return null;
  }
}

async function initializeEditors() {
  const queryFromUrl = getQueryFromUrl();
  const initialValue = queryFromUrl || INITIAL_VALUE;

  inputEditor = monaco.editor.create(document.getElementById("input-editor")!, {
    ...editorConfig,
    value: initialValue,
  });

  outputEditor = monaco.editor.create(
    document.getElementById("output-editor")!,
    {
      ...editorConfig,
      readOnly: true,
    },
  );

  inputEditor.onDidChangeModelContent(() => {
    const query = inputEditor.getValue();
    setQueryInUrl(query);
    formatQuery();
  });

  inputEditor.focus();
  formatQuery();
}

function formatQuery() {
  const query = inputEditor.getValue();

  if (!query) {
    outputEditor.setValue("");
    return;
  }

  try {
    const result = format(query);
    outputEditor.setValue(result || "");
  } catch (error) {
    outputEditor.setValue(`Error: ${error.message}`);
  }
}

initializeEditors();
