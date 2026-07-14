use sigma_data::{DataFrame, Dag, Dashboard};

#[test]
fn test_dataframe_mean_computation() {
    let mut df = DataFrame::new();
    df.add_float_column("price", vec![10.0, 20.0, 30.0]);
    df.add_int_column("count", vec![1, 2, 3]);
    
    assert_eq!(df.row_count, 3);
    let mean = df.mean_of("price").unwrap();
    assert!((mean - 20.0).abs() < 1e-9);
    assert!(df.mean_of("nonexistent").is_none());
}

#[test]
fn test_dag_topological_sort() {
    let mut dag = Dag::new();
    dag.add_node(1, "extract", vec![]);
    dag.add_node(2, "transform", vec![1]);
    dag.add_node(3, "load", vec![2]);

    let order = dag.topological_order().unwrap();
    assert_eq!(order, vec![1, 2, 3]);
}

#[test]
fn test_dashboard_widgets() {
    let mut board = Dashboard::new("SigmaOS Analytics");
    board.add_widget("LineChart");
    board.add_widget("PieChart");
    assert_eq!(board.widgets.len(), 2);
    assert_eq!(board.title, "SigmaOS Analytics");
}
