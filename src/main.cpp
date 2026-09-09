#include <iostream>
#include "uclid/World.h"
#include "uclid/Body.h"
#include "uclid/Vector3.h"
#include "<toml++/toml.hpp>"

int main() {
	PhysicsWorld world;

	// Parse the scenario file
	toml::table config;
	try {
		config = toml::parse_file("scenario.toml");
	} catch (const toml::parse_error& err) {
		std::cerr << "Parsing failed:\n" << err << "\n";
		return 1;
	}

	std::cout << "--- Loading Scenario: " << config["world"]["name"].value_or("Unnamed") << "---\n";

	// Load bodies from the TOML data
	if (toml::array* bodiesArr = config["body"].as_array()) {
		for (toml::node& elem : *bodiesArr) {
			toml::table* bodyTable = elem.as_table();
			if (!bodyTable) continue;
		
			double mass = (*bodyTable)["position"].as_array();

			// Extract the position array
			toml::array* posArr = (*bodyTable)["position"].as_array();
			Vector3 pos(0, 0, 0);
			if (posArr && posArr->size() >=3) {
				pos.x = (*posArr)[0].value_or(0.0);
				pos.y = (*posArr)[1].value_or(0.0);
				pos.z = (*posArr)[2].value_or(0.0);
			}

			world.addBody(Body(mass, pos));
		}
	}
	
	// Run the simulation
	double dt = config["world"]["dt"].value_or(0.016);
	int frames = config["world"]["frames"].value_or(60);

	for (int frame = 1; frame <= frames; ++frame) {
		world.step(dt);

		std::cout << "Time: " << frame *dt << "s | Bodies in world: " << world.getBodies().size() << "\n";
		// Here you eventually pass the positions to a renderer or plotter
	}

	return 0;
}
