import java.net.URL;
import java.util.concurrent.Callable;
import java.util.concurrent.FutureTask;

public class Main {

    public static void main(String[] args) throws Exception {

        System.out.println("Hello World!");

        Callable<Integer> callble = new MyCallable();
        FutureTask futureTask = new FutureTask(callble);
        new Thread(futureTask).start();
        System.out.println(futureTask.get());
    }

    static class MyCallable implements Callable<Integer> {

        @Override
        public Integer call() throws Exception {
            int j = 0;
            for (int i = 0; i < 10; i++, j++) {
                Thread.sleep(500);
            }
            return j;
        }
    }
}
