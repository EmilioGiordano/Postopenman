<script lang="ts">
  let { json }: { json: string } = $props();

  interface Token {
    type: 'key' | 'string' | 'number' | 'bool' | 'null' | 'brace' | 'punct';
    text: string;
  }

  let tokens = $derived.by<Token[]>(() => {
    const result: Token[] = [];
    const src = json;
    let i = 0;

    while (i < src.length) {
      const ch = src[i];

      if (ch === '"') {
        const start = i;
        i++;
        while (i < src.length && src[i] !== '"') {
          if (src[i] === '\\') i++;
          i++;
        }
        i++;
        const text = src.slice(start, i);

        let j = i;
        while (j < src.length && (src[j] === ' ' || src[j] === '\t')) j++;
        if (src[j] === ':') {
          result.push({ type: 'key', text });
        } else {
          result.push({ type: 'string', text });
        }
      } else if (ch === '-' || (ch >= '0' && ch <= '9')) {
        const start = i;
        while (i < src.length && /[\d.eE+\-]/.test(src[i])) i++;
        result.push({ type: 'number', text: src.slice(start, i) });
      } else if (src.startsWith('true', i)) {
        result.push({ type: 'bool', text: 'true' });
        i += 4;
      } else if (src.startsWith('false', i)) {
        result.push({ type: 'bool', text: 'false' });
        i += 5;
      } else if (src.startsWith('null', i)) {
        result.push({ type: 'null', text: 'null' });
        i += 4;
      } else if (ch === '{' || ch === '}' || ch === '[' || ch === ']') {
        result.push({ type: 'brace', text: ch });
        i++;
      } else if (ch === ':' || ch === ',') {
        result.push({ type: 'punct', text: ch });
        i++;
      } else {
        const start = i;
        while (i < src.length && !' "{}[]:,-0123456789tfn'.includes(src[i])) i++;
        if (i === start) {
          result.push({ type: 'punct', text: ch });
          i++;
        } else {
          result.push({ type: 'punct', text: src.slice(start, i) });
        }
      }
    }

    return result;
  });
</script>

<pre class="json-hl">{#each tokens as t}<span class="json-{t.type}">{t.text}</span>{/each}</pre>

<style>
  .json-hl {
    margin: 0;
    font-family: 'Consolas', 'Courier New', monospace;
    font-size: 12px;
    line-height: 1.6;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .json-key {
    color: #8be9fd;
  }

  .json-string {
    color: #f1fa8c;
  }

  .json-number {
    color: #bd93f9;
  }

  .json-bool {
    color: #ffb86c;
  }

  .json-null {
    color: #ff79c6;
  }

  .json-brace {
    color: #f8f8f2;
  }

  .json-punct {
    color: #6272a4;
  }
</style>
