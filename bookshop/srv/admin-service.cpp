// quick_example.cpp
#include <emscripten.h>
#include <emscripten/bind.h>

using namespace emscripten;

// Export phase function to determine execution phase
std::string phase()
{
  return "before";
}

// Export event function to determine trigger event
std::string event()
{
  return "CREATE";
}

// Export entity function to determine target entity name
std::string entity()
{
  return "Authors";
}

// Export exec function to attach to the event
void exec(val req)
{
  // Create CQN object
  val cqn = val::global("CQL")(std::string("SELECT MAX(ID) AS ID FROM AdminService.Authors"));

  // Execute CQN and await the result
  val result = val::global("SELECT")(cqn).await();

  // Extract the id from the result
  int id = result[0]["ID"].as<int>();

  // Read data from the request
  val data = req["data"];

  // Update ID in the request based on the max id
  data.set("ID", id - id % 100 + 100 + 1);
}

EMSCRIPTEN_BINDINGS(module)
{
  function("phase", &phase);
  function("event", &event);
  function("entity", &entity);
  function("exec", &exec);
}
