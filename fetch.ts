import { load } from 'npm:cheerio';

const token = Deno.env.get('AOC_SESSION');
const day = parseInt(Deno.args[0]);
const headers = {
	Cookie: `session=${token}`,
};

const source = await (await fetch(`https://adventofcode.com/2023/day/${day}`, { headers, })).text();
const $ = load(source);
const exampleInput = $('pre > code').first().text();

const input = await (await fetch(`https://adventofcode.com/2023/day/${day}/input`, { headers })).text();

Deno.writeTextFileSync(`inputs/example${day}.txt`, exampleInput);
Deno.writeTextFileSync(`inputs/input${day}.txt`, input);
