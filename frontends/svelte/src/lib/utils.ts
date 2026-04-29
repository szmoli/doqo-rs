import type { DoqoSymbol } from "./bindings/DoqoSymbol";
import type { DoqoRegistry } from "./bindings/DoqoRegistry";

export function symbolName(symbol: DoqoSymbol): string {
  return symbol.fqid.split("::").pop() ?? symbol.fqid
}

export function hasChildren(symbol: DoqoSymbol): boolean {
  return symbol.children.length > 0;
}

export function hasParent(symbol: DoqoSymbol): boolean {
  return symbol.parent ? true : false;
}

export function fqidToPath(fqid: string): string {
  return fqid.replaceAll("::", "/");
}

export function pathToFqid(path: string): string {
  return path.replaceAll("/", "::");
}

export function source(symbol: DoqoSymbol, registry: DoqoRegistry): string {
  const source = registry.sources[symbol.span.source_id];
  return source.content.slice(symbol.span.start, symbol.span.end)
}