import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { OsProvider } from "@/os/OsContext";
import { DriversProvider } from "@/os/DriverContext";
import { OsRoot } from "@/screens/OsRoot";

const queryClient = new QueryClient();

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <OsProvider>
        <DriversProvider>
          <OsRoot />
        </DriversProvider>
      </OsProvider>
    </QueryClientProvider>
  );
}

export default App;
