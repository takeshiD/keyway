const App: React.FC = () => {
  return (
    <div
      data-tauri-drag-region
      className="w-full h-screen flex flex-row"
      style={{
        backgroundColor: "rgba(255, 0, 0, 0.7)",
        borderRadius: 10,
        boxShadow: "10px 10px 15px -10px;"
      }}
    >
      <button className="btn btn-primary">KeyWindow</button>
    </div>
  )
}
export default App;
