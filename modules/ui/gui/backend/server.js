const express = require('express');
const { graphqlHTTP } = require('express-graphql');
const { buildSchema } = require('graphql');

const app = express();
const port = 3000;

// Σ SigmaOS Sovereign API Schema
const schema = buildSchema(`
  type SystemStats {
    cpu: Float
    memory: Int
    activeShards: Int
    uptime: Int
  }

  type Query {
    stats: SystemStats
    shards: [String]
  }
`);

// Mock data reflecting the Sovereign Lattice state
const rootValue = {
  stats: () => ({
    cpu: Math.random() * 100,
    memory: 1024 * 1024,
    activeShards: 500,
    uptime: process.uptime() | 0
  }),
  shards: () => ["S01_Genesis", "S02_ZenithUI", "S30_Supremacy"]
};

// REST Endpoints
app.get('/api/stats', (req, res) => {
  res.json({
    status: "online",
    version: "10.0",
    payload: rootValue.stats()
  });
});

// GraphQL Endpoint
app.use('/graphql', graphqlHTTP({
  schema: schema,
  rootValue: rootValue,
  graphiql: true,
}));

app.listen(port, () => {
  console.log(`Σ SigmaOS External API Bridge running at http://localhost:${port}`);
  console.log(`- REST: http://localhost:${port}/api/stats`);
  console.log(`- GraphQL: http://localhost:${port}/graphql`);
});
