import { load } from 'https://deno.land/std@0.208.0/dotenv/mod.ts';
import cheerio from 'npm:cheerio';

const env = await load();
const token = env.AOC_SESSION;
const day = parseInt(Deno.args[0]);
const headers = {
	Cookie: `session=${token}`,
};

const source = await (await fetch(`https://adventofcode.com/2023/day/${day}`, { headers, })).text();
const $ = cheerio.load(source);
const exampleInput = $('pre > code').first().text();

const input = await (await fetch(`https://adventofcode.com/2023/day/${day}/input`, { headers })).text();

await Deno.writeFile(`inputs/example${day}.txt`, new TextEncoder().encode(exampleInput));
await Deno.writeFile(`inputs/input${day}.txt`, new TextEncoder().encode(input));
