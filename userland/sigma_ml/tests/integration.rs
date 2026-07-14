use sigma_ml::{Tensor, NeuralNetwork, DistributedEngine};

#[test]
fn test_tensor_elementwise_addition() {
    let t1 = Tensor::new(vec![2, 2], vec![1.0, 2.0, 3.0, 4.0]);
    let t2 = Tensor::new(vec![2, 2], vec![5.0, 6.0, 7.0, 8.0]);

    let res = t1.elementwise_add(&t2).unwrap();
    assert_eq!(res.data, vec![6.0, 8.0, 10.0, 12.0]);
    assert_eq!(res.shape, vec![2, 2]);

    let t3 = Tensor::new(vec![3], vec![1.0, 2.0, 3.0]);
    assert!(t1.elementwise_add(&t3).is_err());
}

#[test]
fn test_neural_network_layers() {
    let mut nn = NeuralNetwork::new();
    nn.add_layer(4, 2);
    
    let input = Tensor::new(vec![4], vec![1.0, 0.0, 2.0, -1.0]);
    let out = nn.forward(&input).unwrap();
    assert_eq!(out.shape, vec![2]);
    assert_eq!(out.data, vec![1.0, 1.0]);

    let invalid_input = Tensor::new(vec![3], vec![1.0, 2.0, 3.0]);
    assert!(nn.forward(&invalid_input).is_err());
}

#[test]
fn test_distributed_engine_task_submission() {
    let mut engine = DistributedEngine::new();
    assert_eq!(engine.tasks.len(), 0);

    let task = engine.submit_task(101, "train_mnist");
    assert_eq!(task.id, 101);
    assert_eq!(task.payload, "train_mnist");
    assert_eq!(task.status, "Pending");
    assert_eq!(engine.tasks.len(), 1);
}
