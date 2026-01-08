async function getHealth() {
  try {
    const res = await fetch("http://localhost:8080/health", {
      cache: "no-store",
    });
    return res.json();
  } catch {
    return null;
  }
}

export default async function Home() {
  const health = await getHealth();

  return (
    <main className="container">
      <h1>{{project_name}}</h1>
      <p>Welcome to your new project!</p>

      <section className="health-section">
        <h2>Backend Health</h2>
        {health ? (
          <p className={health.healthy ? "healthy" : "unhealthy"}>
            Status: {health.healthy ? "Healthy" : "Unhealthy"}
          </p>
        ) : (
          <p className="error">Backend unavailable</p>
        )}
      </section>
    </main>
  );
}
