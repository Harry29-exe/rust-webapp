FROM postgres:15.1

#COPY ./initdb /docker-entrypoint-initdb.d
EXPOSE 5432
ENV POSTGRES_PASSWORD=123

CMD ["postgres", "-c", "log_statement=all"]