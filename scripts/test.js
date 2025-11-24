#!/usr/bin/env node

/**
 * DNS Toys Testing Menu Script
 * Interactive menu to test different DNS services
 * Made by AI Assistant
 */

import { createInterface } from "readline";
import { promisify } from "util";
import { exec } from "child_process";
import chalk from "chalk";

const execAsync = promisify(exec);

// Configuration
const DNS_SERVER = "127.0.0.1";
const DNS_PORT = "8053";
const DOMAIN = "localhost";

// Create readline interface
const rl = createInterface({
	input: process.stdin,
	output: process.stdout,
});

// Helper function to prompt for input
const question = (query) => {
	return new Promise((resolve) => {
		rl.question(query, resolve);
	});
};

// Function to print colored output
const printColor = (colorFn, message) => {
	console.log(colorFn(message));
};

// Function to test a DNS query
const testDns = async (query, recordType = "TXT", description) => {
	printColor(chalk.cyan, `Testing: ${description}`);
	printColor(chalk.yellow, `Query: ${query}`);
	printColor(
		chalk.blue,
		`Command: dig @${DNS_SERVER} -p ${DNS_PORT} ${query} ${recordType}`,
	);
	console.log("----------------------------------------");

	try {
		const { stdout, stderr } = await execAsync(
			`dig @${DNS_SERVER} -p ${DNS_PORT} ${query} ${recordType}`,
		);
		console.log(stdout);
		if (stderr) {
			console.error(stderr);
		}
	} catch (error) {
		printColor(chalk.red, `Error executing dig: ${error.message}`);
	}

	console.log("");
	await question("Press Enter to continue...");
	printColor(chalk.magenta, "To exit the test menu at any time, press Ctrl+C.");
};

// Function to show help
const showHelp = () => {
	printColor(chalk.green, "Available DNS Services:");
	console.log("1. IP Service - Get your IP address");
	console.log("2. UUID Service - Generate UUIDs");
	console.log("3. Pi Service - Get Pi constant");
	console.log("4. Geo Service - Get location information");
	console.log("5. Random Service - Generate random numbers");
	console.log("6. Help Service - Show available services");
	console.log("7. Test All Services");
	console.log("8. Custom Query");
	console.log("9. Exit");
	console.log("");
};

// Function to test IP service
const testIpService = async () => {
	printColor(chalk.green, "=== IP Service Tests ===");
	await testDns(`ip.${DOMAIN}`, "TXT", "IP as TXT record");
	await testDns(`ip.${DOMAIN}`, "A", "IP as A record");
};

// Function to test UUID service
const testUuidService = async () => {
	printColor(chalk.green, "=== UUID Service Tests ===");
	await testDns(`1.uuid.${DOMAIN}`, "TXT", "Single UUID");
	await testDns(`5.uuid.${DOMAIN}`, "TXT", "5 UUIDs");
	await testDns(`10.uuid.${DOMAIN}`, "TXT", "10 UUIDs");
};

// Function to test Pi service
const testPiService = async () => {
	printColor(chalk.green, "=== Pi Service Tests ===");
	await testDns(`pi.${DOMAIN}`, "TXT", "Pi as text");
	await testDns(`pi.${DOMAIN}`, "A", "Pi as IPv4 address");
	await testDns(`pi.${DOMAIN}`, "AAAA", "Pi as IPv6 address");
};

// Function to test Geo service
const testGeoService = async () => {
	printColor(chalk.green, "=== Geo Service Tests ===");
	await testDns(`mumbai.geo.${DOMAIN}`, "TXT", "Mumbai location info");
	await testDns(`london.geo.${DOMAIN}`, "TXT", "London location info");
	await testDns(`new york.geo.${DOMAIN}`, "TXT", "New York location info");
	await testDns(`tokyo.geo.${DOMAIN}`, "TXT", "Tokyo location info");
	await testDns(`london/uk.geo.${DOMAIN}`, "TXT", "London, UK specifically");
};

// Function to test Random service
const testRandomService = async () => {
	printColor(chalk.green, "=== Random Service Tests ===");
	await testDns(`1-100.random.${DOMAIN}`, "TXT", "Random number 1-100");
	await testDns(`1-10.random.${DOMAIN}`, "TXT", "Random number 1-10");
	await testDns(`50-100.random.${DOMAIN}`, "TXT", "Random number 50-100");
};

// Function to test Help service
const testHelpService = async () => {
	printColor(chalk.green, "=== Help Service Tests ===");
	await testDns(`help.${DOMAIN}`, "TXT", "Show available services");
};

// Function to test all services
const testAllServices = async () => {
	printColor(chalk.magenta, "=== Testing All Services ===");
	await testIpService();
	await testUuidService();
	await testPiService();
	await testGeoService();
	await testRandomService();
	await testHelpService();
};

// Function to test custom query
const testCustomQuery = async () => {
	printColor(chalk.green, "=== Custom Query ===");
	const query = await question("Enter DNS query (e.g., ip.localhost): ");
	const recordType =
		(await question("Enter record type (default: TXT): ")) || "TXT";
	await testDns(query, recordType, `Custom query: ${query}`);
};

// Function to check if DNS server is running
const checkDnsServer = async () => {
	printColor(chalk.yellow, "Checking if DNS server is running...");
	try {
		await execAsync(
			`dig @${DNS_SERVER} -p ${DNS_PORT} help.${DOMAIN} TXT +timeout=2 +tries=1`,
			{ timeout: 3000 },
		);
		printColor(
			chalk.green,
			`✅ DNS server is running on ${DNS_SERVER}:${DNS_PORT}`,
		);
		return true;
	} catch (error) {
		printColor(
			chalk.red,
			`❌ DNS server is not running on ${DNS_SERVER}:${DNS_PORT}`,
		);
		printColor(chalk.yellow, "Please start the server with: just run");
		return false;
	}
};

// Main menu loop
const mainMenu = async () => {
	while (true) {
		console.clear();
		printColor(chalk.magenta, "==========================================");
		printColor(chalk.magenta, "        DNS Toys Testing Menu");
		printColor(chalk.magenta, "==========================================");
		console.log("");

		showHelp();

		const choice = await question("Choose an option (1-9): ");

		switch (choice) {
			case "1":
				await testIpService();
				break;
			case "2":
				await testUuidService();
				break;
			case "3":
				await testPiService();
				break;
			case "4":
				await testGeoService();
				break;
			case "5":
				await testRandomService();
				break;
			case "6":
				await testHelpService();
				break;
			case "7":
				await testAllServices();
				break;
			case "8":
				await testCustomQuery();
				break;
			case "9":
				printColor(chalk.green, "Goodbye! 👋");
				rl.close();
				process.exit(0);
				break;
			default:
				printColor(chalk.red, "Invalid option. Please choose 1-9.");
				await question("Press Enter to continue...");
				break;
		}
	}
};

// Main execution
const main = async () => {
	printColor(chalk.blue, "DNS Toys Testing Menu");
	printColor(chalk.blue, "====================");
	console.log("");

	// Check if DNS server is running
	if (!(await checkDnsServer())) {
		rl.close();
		process.exit(1);
	}

	console.log("");
	await question("Press Enter to start the menu...");

	await mainMenu();
};

// Handle Ctrl+C gracefully
process.on("SIGINT", () => {
	console.log("\n");
	printColor(chalk.yellow, "Exiting...");
	rl.close();
	process.exit(0);
});

// Run the main function
main().catch((error) => {
	printColor(chalk.red, `Error: ${error.message}`);
	rl.close();
	process.exit(1);
});
