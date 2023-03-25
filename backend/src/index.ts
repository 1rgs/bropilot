export { };

addEventListener("fetch", (event) => {
	event.respondWith(handleRequest(event.request));
});

type OpenAIResult = {
	choices: {
		message: {
			content: string;
		};
	}[];
};

declare global {
	const OPENAI_KEY: string;
}

async function handleRequest(request: Request): Promise<Response> {
	const url = new URL(request.url);
	const apiKey = OPENAI_KEY;
	const query = url.searchParams.get("query");
	const context = url.searchParams.get("context") || undefined;

	if (!apiKey || !query) {
		return new Response("Bad Request", { status: 400 });
	}

	const prompt = constructGptPrompt(decodeURIComponent(query), context && decodeURIComponent(context));
	const openaiRequestBody = JSON.stringify({
		model: "gpt-3.5-turbo",
		messages: [{ role: "user", content: prompt }],
		temperature: 0.7,
	});

	const openaiResponse = await fetch(
		"https://api.openai.com/v1/chat/completions",
		{
			method: "POST",
			headers: {
				"Content-Type": "application/json",
				Authorization: `Bearer ${apiKey}`,
			},
			body: openaiRequestBody,
		}
	);

	const responseText = await openaiResponse.text();
	console.log(responseText)
	const openaiResult = JSON.parse(responseText) as OpenAIResult;
	const message = openaiResult.choices[0].message.content;
	const { command, explanation } = JSON.parse(message);

	return new Response(JSON.stringify({ command, explanation }), {
		headers: { "Content-Type": "application/json" },
	});
}

function constructGptPrompt(query: string, context?: string): string {
	const outputInstructions =
		`Your response must be of type JSON with schema {"command": "<command>", "explanation": "<explanation>"} where <command> is the ${context ? "rewritten " : ""} command and <explanation> is a short explanation of the change.`;

	if (context) {
		return ` Command: ${context}
		Feedback: ${query}
		
		Rewrite the above bash command to incorporate the requested change. ${outputInstructions}`;
	} else {
		return `
		Write a bash command that does the following:
		Intent: ${query}
		
		${outputInstructions} `;
	}
}
